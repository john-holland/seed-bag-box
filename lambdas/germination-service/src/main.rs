use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use models::{
    GerminationGuide, GerminationObservation, GerminationPhase, GerminationRecord,
    GrowingMedium, ShipmentType, SproutHealthStatus, SproutShipmentPackage,
};

mod fruit_species;
use fruit_species::is_fruit_bearing_species;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StartGerminationRequest {
    seed_id: Uuid,
    customer_id: Uuid,
    species: String,
    variety: Option<String>,
    growing_medium: GrowingMedium,
    shipment_type: ShipmentType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecordObservationRequest {
    germination_record_id: Uuid,
    observed_by: String,
    root_length_mm: Option<f32>,
    shoot_length_mm: Option<f32>,
    leaf_count: Option<u32>,
    health_status: SproutHealthStatus,
    temperature_celsius: Option<f32>,
    humidity_percent: Option<f32>,
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdatePhaseRequest {
    germination_record_id: Uuid,
    new_phase: GerminationPhase,
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrepareShipmentRequest {
    germination_record_ids: Vec<Uuid>,
    customer_id: Uuid,
    expedited: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GerminationResponse {
    germination_record_id: Uuid,
    seed_id: Uuid,
    species: String,
    phase: GerminationPhase,
    germination_success: bool,
    health_status: SproutHealthStatus,
    days_since_started: i64,
    ready_for_shipment: bool,
    estimated_ship_date: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ShipmentPackageResponse {
    package_id: Uuid,
    sprout_count: u32,
    expedited: bool,
    estimated_ship_date: Option<String>,
    care_instructions_url: String,
}

async fn function_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Processing germination request");

    let path = event
        .payload
        .path
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("");
    let method = event.payload.http_method;

    let response = match (method.as_str(), path) {
        ("POST", "/germination/start") => start_germination(event.payload).await?,
        ("POST", "/germination/observe") => record_observation(event.payload).await?,
        ("PUT", "/germination/phase") => update_phase(event.payload).await?,
        ("POST", "/germination/shipment") => prepare_shipment(event.payload).await?,
        ("GET", "/germination/ready") => list_ready_for_shipment().await?,
        ("GET", path) if path.starts_with("/germination/guide/") => {
            get_germination_guide(path).await?
        }
        ("GET", path) if path.starts_with("/germination/") => {
            get_germination_record(path).await?
        }
        _ => ApiGatewayProxyResponse {
            status_code: 404,
            headers: Default::default(),
            multi_value_headers: Default::default(),
            body: Some(Body::Text(r#"{"error": "Not found"}"#.to_string())),
            is_base64_encoded: false,
        },
    };

    Ok(response)
}

async fn start_germination(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: StartGerminationRequest = serde_json::from_str(&body)?;

    let record_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    // Fetch germination guide for species (TODO: from DynamoDB)
    let guide = get_species_guide(&req.species).await?;

    let record = GerminationRecord {
        id: record_id,
        seed_id: req.seed_id,
        plant_id: None,
        customer_id: req.customer_id,
        species: req.species.clone(),
        variety: req.variety,
        germination_phase: GerminationPhase::Imbibition,
        started_at: now,
        imbibition_started_at: now,
        radicle_emerged_at: None,
        shoot_emerged_at: None,
        cotyledon_expanded_at: None,
        true_leaf_emerged_at: None,
        photosynthesis_started_at: None,
        ready_for_shipment_at: None,
        growing_medium: req.growing_medium,
        temperature_celsius: None,
        humidity_percent: None,
        light_hours_per_day: None,
        germination_success: false, // Will be updated when radicle emerges
        health_status: SproutHealthStatus::Good,
        root_length_mm: None,
        shoot_length_mm: None,
        cotyledon_count: None,
        true_leaf_count: None,
        total_leaf_count: None,
        is_true_plant: false,
        is_autotrophic: false,
        has_edible_fruit_potential: Some(is_fruit_bearing_species(&req.species)),
        shipment_type: req.shipment_type,
        estimated_ship_date: Some(
            now + chrono::Duration::days(guide.ready_to_ship_days as i64),
        ),
        actual_ship_date: None,
        customer_instructions: Some(guide.customer_care_instructions.clone()),
        notes: None,
    };

    info!(
        "Started germination record {} for seed {} ({})",
        record_id, req.seed_id, req.species
    );

    // TODO: Save to DynamoDB

    let response = GerminationResponse {
        germination_record_id: record.id,
        seed_id: record.seed_id,
        species: record.species,
        phase: record.germination_phase,
        germination_success: record.germination_success,
        health_status: record.health_status,
        days_since_started: 0,
        ready_for_shipment: false,
        estimated_ship_date: record.estimated_ship_date.map(|d| d.to_rfc3339()),
    };

    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn record_observation(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: RecordObservationRequest = serde_json::from_str(&body)?;

    let observation_id = Uuid::new_v4();
    let observation = GerminationObservation {
        id: observation_id,
        germination_record_id: req.germination_record_id,
        observed_at: chrono::Utc::now(),
        observed_by: req.observed_by,
        root_length_mm: req.root_length_mm,
        shoot_length_mm: req.shoot_length_mm,
        cotyledon_count: None, // TODO: Add to request
        true_leaf_count: None, // TODO: Add to request
        total_leaf_count: None,
        cotyledon_color: None,
        true_leaf_color: None,
        health_status: req.health_status,
        radicle_visible: false, // TODO: Add to request
        shoot_visible: false,
        cotyledons_expanded: false,
        true_leaves_present: false,
        appears_autotrophic: false,
        temperature_celsius: req.temperature_celsius,
        humidity_percent: req.humidity_percent,
        issues_noted: vec![],
        actions_taken: vec![],
        notes: req.notes,
        photo_url: None,
    };

    info!(
        "Recorded observation {} for germination record {}",
        observation_id, req.germination_record_id
    );

    // TODO: Save observation to DynamoDB
    // TODO: Update germination record with latest measurements
    // TODO: Check if ready for shipment based on criteria

    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&observation)?)),
        is_base64_encoded: false,
    })
}

async fn update_phase(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: UpdatePhaseRequest = serde_json::from_str(&body)?;

    info!(
        "Updating germination record {} to phase {:?}",
        req.germination_record_id, req.new_phase
    );

    // TODO: Fetch current record from DynamoDB
    // TODO: Update phase and related timestamps
    // TODO: If phase is ReadyForShipment, trigger shipment preparation

    let now = chrono::Utc::now();

    // Mock response
    let response = serde_json::json!({
        "germination_record_id": req.germination_record_id,
        "new_phase": req.new_phase,
        "updated_at": now.to_rfc3339(),
        "ready_for_shipment": req.new_phase.is_shippable(),
    });

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn prepare_shipment(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: PrepareShipmentRequest = serde_json::from_str(&body)?;

    let package_id = Uuid::new_v4();

    // Determine optimal shipping days (Mon-Wed for live plants)
    let ship_days = vec![
        models::Weekday::Monday,
        models::Weekday::Tuesday,
        models::Weekday::Wednesday,
    ];

    let package = SproutShipmentPackage {
        id: package_id,
        germination_records: req.germination_record_ids.clone(),
        customer_id: req.customer_id,
        shipment_cycle_id: None,
        package_type: models::PackageType::VentilatedBox,
        container_count: 1,
        total_sprouts: req.germination_record_ids.len() as u32,
        moisture_retention: true,
        temperature_control: true, // Ice pack for warm weather
        ventilation: true,
        expedited_shipping_required: req.expedited,
        ship_on_days: ship_days,
        max_transit_days: if req.expedited { 2 } else { 3 },
        care_instructions_included: true,
        transplant_instructions_included: true,
        species_info_card_included: true,
        packed_at: None,
        shipped_at: None,
        expected_delivery: None,
    };

    info!(
        "Prepared shipment package {} with {} sprouts for customer {}",
        package_id,
        req.germination_record_ids.len(),
        req.customer_id
    );

    // TODO: Save package to DynamoDB
    // TODO: Create shipment in shipping service
    // TODO: Generate packing list and care instructions

    let response = ShipmentPackageResponse {
        package_id: package.id,
        sprout_count: package.total_sprouts,
        expedited: package.expedited_shipping_required,
        estimated_ship_date: Some(chrono::Utc::now().to_rfc3339()),
        care_instructions_url: format!("/germination/care-instructions/{}", package.id),
    };

    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn list_ready_for_shipment() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing all sprouts ready for shipment");

    // TODO: Query DynamoDB for records with phase = ReadyForShipment
    // TODO: Group by customer
    // TODO: Check shipment criteria

    let response = serde_json::json!({
        "ready_count": 0,
        "records": []
    });

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn get_germination_record(path: &str) -> Result<ApiGatewayProxyResponse, Error> {
    let id = path.trim_start_matches("/germination/");

    info!("Fetching germination record {}", id);

    // TODO: Fetch from DynamoDB

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(
            format!(r#"{{"germination_record_id": "{}"}}"#, id),
        )),
        is_base64_encoded: false,
    })
}

async fn get_germination_guide(path: &str) -> Result<ApiGatewayProxyResponse, Error> {
    let species = path.trim_start_matches("/germination/guide/");

    info!("Fetching germination guide for {}", species);

    let guide = get_species_guide(species).await?;

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&guide)?)),
        is_base64_encoded: false,
    })
}

/// Get species-specific germination guide
async fn get_species_guide(species: &str) -> Result<GerminationGuide, Error> {
    // TODO: Fetch from DynamoDB
    // For now, return a sample guide based on species

    let guide = match species.to_lowercase().as_str() {
        "tomato" => GerminationGuide {
            species: "tomato".to_string(),
            variety: None,
            imbibition_days: 1,
            radicle_emergence_days_min: 3,
            radicle_emergence_days_max: 7,
            shoot_emergence_days_min: 5,
            shoot_emergence_days_max: 10,
            cotyledon_expansion_days_min: 7,
            cotyledon_expansion_days_max: 12,
            true_leaf_emergence_days_min: 10,
            true_leaf_emergence_days_max: 16,
            photosynthesis_days_min: 12,
            photosynthesis_days_max: 18,
            ready_to_ship_days: 14,
            optimal_temperature_celsius: models::TemperatureRange {
                min: 18.0,
                max: 29.0,
                optimal: 24.0,
            },
            optimal_humidity_percent: models::HumidityRange {
                min: 60.0,
                max: 80.0,
                optimal: 70.0,
            },
            light_requirement: models::LightRequirement::High,
            preferred_medium: vec![
                GrowingMedium::Peat,
                GrowingMedium::Soil,
                GrowingMedium::Rockwool,
            ],
            min_root_length_mm: 20.0,
            min_shoot_length_mm: 30.0,
            min_leaf_count: 2,
            pre_soak_required: false,
            pre_soak_hours: None,
            scarification_required: false,
            stratification_required: false,
            stratification_days: None,
            planting_depth_mm: 6.0,
            spacing_cm: 45.0,
            days_to_maturity: 70,
            customer_care_instructions: "Transplant to 4-inch pot or garden after hardening off. Keep soil moist but not waterlogged. Provide full sun (6-8 hours). Fertilize weekly with balanced fertilizer.".to_string(),
        },
        "cantaloupe" | "cantelope" => GerminationGuide {
            species: "cantaloupe".to_string(),
            variety: None,
            imbibition_days: 1,
            radicle_emergence_days_min: 3,
            radicle_emergence_days_max: 8,
            shoot_emergence_days_min: 5,
            shoot_emergence_days_max: 12,
            cotyledon_expansion_days_min: 7,
            cotyledon_expansion_days_max: 14,
            true_leaf_emergence_days_min: 10,
            true_leaf_emergence_days_max: 18,
            photosynthesis_days_min: 12,
            photosynthesis_days_max: 20,
            ready_to_ship_days: 14,
            optimal_temperature_celsius: models::TemperatureRange {
                min: 21.0,
                max: 32.0,
                optimal: 27.0,
            },
            optimal_humidity_percent: models::HumidityRange {
                min: 60.0,
                max: 80.0,
                optimal: 70.0,
            },
            light_requirement: models::LightRequirement::High,
            preferred_medium: vec![GrowingMedium::Peat, GrowingMedium::Soil],
            min_root_length_mm: 25.0,
            min_shoot_length_mm: 35.0,
            min_true_leaf_count: 2,
            must_be_true_plant: true,
            must_be_autotrophic: false,
            pre_soak_required: false,
            pre_soak_hours: None,
            scarification_required: false,
            stratification_required: false,
            stratification_days: None,
            planting_depth_mm: 12.0,
            spacing_cm: 90.0,
            days_to_maturity: 80,
            customer_care_instructions: "Melons need warmth and space. Transplant after frost. Water deeply but infrequently. Provide full sun. Fertilize when vines start running.".to_string(),
        },
        "watermelon" => GerminationGuide {
            species: "watermelon".to_string(),
            variety: None,
            imbibition_days: 1,
            radicle_emergence_days_min: 3,
            radicle_emergence_days_max: 10,
            shoot_emergence_days_min: 6,
            shoot_emergence_days_max: 14,
            cotyledon_expansion_days_min: 8,
            cotyledon_expansion_days_max: 16,
            true_leaf_emergence_days_min: 12,
            true_leaf_emergence_days_max: 21,
            photosynthesis_days_min: 14,
            photosynthesis_days_max: 24,
            ready_to_ship_days: 16,
            optimal_temperature_celsius: models::TemperatureRange {
                min: 21.0,
                max: 35.0,
                optimal: 27.0,
            },
            optimal_humidity_percent: models::HumidityRange {
                min: 60.0,
                max: 80.0,
                optimal: 70.0,
            },
            light_requirement: models::LightRequirement::High,
            preferred_medium: vec![GrowingMedium::Peat, GrowingMedium::Soil],
            min_root_length_mm: 30.0,
            min_shoot_length_mm: 40.0,
            min_true_leaf_count: 2,
            must_be_true_plant: true,
            must_be_autotrophic: false,
            pre_soak_required: true,
            pre_soak_hours: Some(6),
            scarification_required: false,
            stratification_required: false,
            stratification_days: None,
            planting_depth_mm: 25.0,
            spacing_cm: 180.0,
            days_to_maturity: 90,
            customer_care_instructions: "Watermelons need lots of heat and space. Plant after soil warms to 70°F. Water deeply, especially during fruiting. Full sun required.".to_string(),
        },
        "cabbage" => GerminationGuide {
            species: "cabbage".to_string(),
            variety: None,
            imbibition_days: 1,
            radicle_emergence_days_min: 4,
            radicle_emergence_days_max: 10,
            shoot_emergence_days_min: 6,
            shoot_emergence_days_max: 12,
            cotyledon_expansion_days_min: 8,
            cotyledon_expansion_days_max: 14,
            true_leaf_emergence_days_min: 10,
            true_leaf_emergence_days_max: 18,
            photosynthesis_days_min: 12,
            photosynthesis_days_max: 20,
            ready_to_ship_days: 14,
            optimal_temperature_celsius: models::TemperatureRange {
                min: 15.0,
                max: 24.0,
                optimal: 20.0,
            },
            optimal_humidity_percent: models::HumidityRange {
                min: 60.0,
                max: 75.0,
                optimal: 68.0,
            },
            light_requirement: models::LightRequirement::High,
            preferred_medium: vec![GrowingMedium::Peat, GrowingMedium::Soil],
            min_root_length_mm: 20.0,
            min_shoot_length_mm: 30.0,
            min_true_leaf_count: 2,
            must_be_true_plant: true,
            must_be_autotrophic: false,
            pre_soak_required: false,
            pre_soak_hours: None,
            scarification_required: false,
            stratification_required: false,
            stratification_days: None,
            planting_depth_mm: 6.0,
            spacing_cm: 45.0,
            days_to_maturity: 70,
            customer_care_instructions: "Cabbage is cool-season crop. Transplant 2-3 weeks before last frost. Keep soil moist. Tolerates light frost. Harvest when heads are firm.".to_string(),
        },
        "wheat" => GerminationGuide {
            species: "wheat".to_string(),
            variety: None,
            imbibition_days: 2,
            radicle_emergence_days_min: 3,
            radicle_emergence_days_max: 7,
            shoot_emergence_days_min: 5,
            shoot_emergence_days_max: 10,
            cotyledon_expansion_days_min: 7,
            cotyledon_expansion_days_max: 12,
            true_leaf_emergence_days_min: 10,
            true_leaf_emergence_days_max: 14,
            photosynthesis_days_min: 12,
            photosynthesis_days_max: 16,
            ready_to_ship_days: 10,
            optimal_temperature_celsius: models::TemperatureRange {
                min: 12.0,
                max: 24.0,
                optimal: 18.0,
            },
            optimal_humidity_percent: models::HumidityRange {
                min: 50.0,
                max: 70.0,
                optimal: 60.0,
            },
            light_requirement: models::LightRequirement::High,
            preferred_medium: vec![GrowingMedium::Soil],
            min_root_length_mm: 25.0,
            min_shoot_length_mm: 50.0,
            min_true_leaf_count: 1,
            must_be_true_plant: true,
            must_be_autotrophic: false,
            pre_soak_required: false,
            pre_soak_hours: None,
            scarification_required: false,
            stratification_required: false,
            stratification_days: None,
            planting_depth_mm: 25.0,
            spacing_cm: 5.0,
            days_to_maturity: 120,
            customer_care_instructions: "Direct sow wheat in fall or spring. Plant densely for grain production. Water during establishment, then reduce. Harvest when golden and heads droop.".to_string(),
        },
        "sugar cane" | "sugarcane" | "cane sugar" => GerminationGuide {
            species: "sugar_cane".to_string(),
            variety: None,
            imbibition_days: 3,
            radicle_emergence_days_min: 7,
            radicle_emergence_days_max: 14,
            shoot_emergence_days_min: 10,
            shoot_emergence_days_max: 21,
            cotyledon_expansion_days_min: 14,
            cotyledon_expansion_days_max: 28,
            true_leaf_emergence_days_min: 21,
            true_leaf_emergence_days_max: 35,
            photosynthesis_days_min: 28,
            photosynthesis_days_max: 42,
            ready_to_ship_days: 30,
            optimal_temperature_celsius: models::TemperatureRange {
                min: 24.0,
                max: 35.0,
                optimal: 30.0,
            },
            optimal_humidity_percent: models::HumidityRange {
                min: 70.0,
                max: 85.0,
                optimal: 78.0,
            },
            light_requirement: models::LightRequirement::High,
            preferred_medium: vec![GrowingMedium::Soil],
            min_root_length_mm: 40.0,
            min_shoot_length_mm: 100.0,
            min_true_leaf_count: 3,
            must_be_true_plant: true,
            must_be_autotrophic: true,
            pre_soak_required: true,
            pre_soak_hours: Some(12),
            scarification_required: false,
            stratification_required: false,
            stratification_days: None,
            planting_depth_mm: 50.0,
            spacing_cm: 120.0,
            days_to_maturity: 365,
            customer_care_instructions: "Sugar cane needs tropical conditions. Requires lots of water and full sun. Plant stem cuttings with nodes. Takes 12+ months to mature. Harvest when stalks are thick.".to_string(),
        },
        "cannabis" | "marijuana" | "hemp" => GerminationGuide {
            species: "cannabis".to_string(),
            variety: None,
            imbibition_days: 1,
            radicle_emergence_days_min: 2,
            radicle_emergence_days_max: 5,
            shoot_emergence_days_min: 3,
            shoot_emergence_days_max: 7,
            cotyledon_expansion_days_min: 5,
            cotyledon_expansion_days_max: 10,
            true_leaf_emergence_days_min: 7,
            true_leaf_emergence_days_max: 12,
            photosynthesis_days_min: 10,
            photosynthesis_days_max: 14,
            ready_to_ship_days: 14,
            optimal_temperature_celsius: models::TemperatureRange {
                min: 20.0,
                max: 30.0,
                optimal: 25.0,
            },
            optimal_humidity_percent: models::HumidityRange {
                min: 50.0,
                max: 70.0,
                optimal: 60.0,
            },
            light_requirement: models::LightRequirement::Photoperiod { hours_per_day: 18.0 },
            preferred_medium: vec![GrowingMedium::Soil, GrowingMedium::Coco, GrowingMedium::Rockwool],
            min_root_length_mm: 25.0,
            min_shoot_length_mm: 40.0,
            min_true_leaf_count: 3,
            must_be_true_plant: true,
            must_be_autotrophic: true,
            pre_soak_required: true,
            pre_soak_hours: Some(12),
            scarification_required: false,
            stratification_required: false,
            stratification_days: None,
            planting_depth_mm: 6.0,
            spacing_cm: 100.0,
            days_to_maturity: 90,
            customer_care_instructions: "⚠️ LEGAL COMPLIANCE REQUIRED - Check state/federal laws before growing. Requires 18-24hr light for vegetative growth. Strict phenotype separation. Excellent drainage essential. pH 6.0-7.0.".to_string(),
        },
        "basil" => GerminationGuide {
            species: "basil".to_string(),
            variety: None,
            imbibition_days: 1,
            radicle_emergence_days_min: 3,
            radicle_emergence_days_max: 7,
            shoot_emergence_days_min: 5,
            shoot_emergence_days_max: 9,
            cotyledon_expansion_days_min: 6,
            cotyledon_expansion_days_max: 10,
            true_leaf_emergence_days_min: 8,
            true_leaf_emergence_days_max: 12,
            photosynthesis_days_min: 10,
            photosynthesis_days_max: 14,
            ready_to_ship_days: 12,
            optimal_temperature_celsius: models::TemperatureRange {
                min: 20.0,
                max: 30.0,
                optimal: 25.0,
            },
            optimal_humidity_percent: models::HumidityRange {
                min: 50.0,
                max: 70.0,
                optimal: 60.0,
            },
            light_requirement: models::LightRequirement::High,
            preferred_medium: vec![GrowingMedium::Peat, GrowingMedium::Soil],
            min_root_length_mm: 15.0,
            min_shoot_length_mm: 25.0,
            min_true_leaf_count: 2,
            must_be_true_plant: true,
            must_be_autotrophic: true,
            pre_soak_required: false,
            pre_soak_hours: None,
            scarification_required: false,
            stratification_required: false,
            stratification_days: None,
            planting_depth_mm: 3.0,
            spacing_cm: 20.0,
            days_to_maturity: 60,
            customer_care_instructions: "Keep warm (above 50°F). Water when soil surface is dry. Pinch growing tips to encourage bushiness. Harvest leaves regularly.".to_string(),
        },
        _ => GerminationGuide {
            species: species.to_string(),
            variety: None,
            imbibition_days: 2,
            radicle_emergence_days_min: 5,
            radicle_emergence_days_max: 10,
            shoot_emergence_days_min: 7,
            shoot_emergence_days_max: 14,
            cotyledon_expansion_days_min: 10,
            cotyledon_expansion_days_max: 18,
            true_leaf_emergence_days_min: 14,
            true_leaf_emergence_days_max: 24,
            photosynthesis_days_min: 16,
            photosynthesis_days_max: 28,
            ready_to_ship_days: 21,
            optimal_temperature_celsius: models::TemperatureRange {
                min: 18.0,
                max: 25.0,
                optimal: 21.0,
            },
            optimal_humidity_percent: models::HumidityRange {
                min: 60.0,
                max: 80.0,
                optimal: 70.0,
            },
            light_requirement: models::LightRequirement::Medium,
            preferred_medium: vec![GrowingMedium::Soil, GrowingMedium::Peat],
            min_root_length_mm: 20.0,
            min_shoot_length_mm: 30.0,
            min_leaf_count: 2,
            pre_soak_required: false,
            pre_soak_hours: None,
            scarification_required: false,
            stratification_required: false,
            stratification_days: None,
            planting_depth_mm: 6.0,
            spacing_cm: 30.0,
            days_to_maturity: 90,
            customer_care_instructions: "Transplant carefully. Water regularly. Provide appropriate light for species.".to_string(),
        },
    };

    Ok(guide)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

