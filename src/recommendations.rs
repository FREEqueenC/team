use calamine::{open_workbook, Reader, Xlsx};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub id: f64,
    pub project_name: String,
    pub project_description: String,
    pub country: String,
    pub naics_sector: String,
    pub committed: f64,
    pub department: String,
    pub project_type: String,
    pub region: String,
    pub fiscal_year: String,
    pub project_number: String,
    pub framework: String,
    pub project_profile_url: String,
    pub tags: Vec<String>,
    pub starred: bool,
    pub comment: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RecommendationRequest {
    pub preferences: Vec<String>,
}

fn get_preference_to_filter_mappings() -> HashMap<String, serde_json::Value> {
    let mut mappings = HashMap::new();
    mappings.insert("Agriculture".to_string(), serde_json::json!({ "naicsSectors": ["Agriculture"], "departments": ["Technical Assistance"] }));
    mappings.insert("Education".to_string(), serde_json::json!({ "naicsSectors": ["Educational Services"], "departments": ["Technical Assistance"] }));
    mappings.insert("Healthcare Access".to_string(), serde_json::json!({ "naicsSectors": ["Health Care"], "departments": ["Equity Investments"] }));
    mappings.insert("Financial Inclusion".to_string(), serde_json::json!({ "naicsSectors": ["Finance and Insurance"], "departments": ["Investment Funds", "Finance"] }));
    mappings.insert(
        "Infrastructure Development".to_string(),
        serde_json::json!({ "naicsSectors": ["Utilities"], "departments": ["Finance"] }),
    );
    mappings.insert(
        "Technology Innovation".to_string(),
        serde_json::json!({ "naicsSectors": ["Information"], "departments": ["Investment Funds"] }),
    );
    mappings.insert("Small Business Support".to_string(), serde_json::json!({ "naicsSectors": ["Finance and Insurance"], "departments": ["Investment Funds"] }));
    mappings.insert(
        "Rural Development".to_string(),
        serde_json::json!({ "departments": ["Technical Assistance"] }),
    );
    mappings.insert(
        "Environmental Sustainability".to_string(),
        serde_json::json!({ "naicsSectors": ["Utilities"], "departments": ["Finance"] }),
    );
    mappings.insert(
        "Renewable Energy".to_string(),
        serde_json::json!({ "naicsSectors": ["Utilities"], "departments": ["Finance"] }),
    );
    mappings.insert(
        "Water & Sanitation".to_string(),
        serde_json::json!({ "naicsSectors": ["Utilities"], "departments": ["Finance"] }),
    );
    mappings.insert("Digital Inclusion".to_string(), serde_json::json!({ "naicsSectors": ["Information", "Educational Services"], "departments": ["Technical Assistance"] }));
    mappings.insert("Economic Growth".to_string(), serde_json::json!({ "naicsSectors": ["Finance and Insurance"], "departments": ["Investment Funds"] }));
    mappings.insert("Food Security".to_string(), serde_json::json!({ "naicsSectors": ["Agriculture"], "departments": ["Technical Assistance"] }));
    mappings
}

fn find_column_index(headers: &[String], possible_names: &[&str]) -> Option<usize> {
    for name in possible_names {
        if let Some(index) = headers
            .iter()
            .position(|h| h.to_lowercase().contains(&name.to_lowercase()))
        {
            return Some(index);
        }
    }
    None
}

pub fn get_recommendations(
    preferences: &[String],
    excel_file_path: &str,
) -> Result<Vec<Project>, anyhow::Error> {
    let mut excel: Xlsx<_> = open_workbook(excel_file_path)?;
    let mut projects = Vec::new();

    if let Some(Ok(range)) = excel.worksheet_range_at(0) {
        let mut rows = range.rows();
        let headers = rows
            .next()
            .unwrap()
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        // Find column indices dynamically using multiple possible column names
        let project_name_idx =
            find_column_index(&headers, &["project name", "name", "title", "project"]);
        let project_description_idx = find_column_index(
            &headers,
            &["project description", "description", "desc", "summary"],
        );
        let country_idx = find_column_index(&headers, &["country", "nation", "location country"]);
        let naics_sector_idx = find_column_index(
            &headers,
            &[
                "naics sector",
                "sector",
                "industry",
                "naics",
                "industry sector",
            ],
        );
        let committed_idx = find_column_index(
            &headers,
            &["committed", "amount", "funding", "budget", "cost"],
        );
        let department_idx = find_column_index(
            &headers,
            &["department", "dept", "division", "unit", "team"],
        );
        let project_type_idx =
            find_column_index(&headers, &["project type", "type", "category", "kind"]);
        let region_idx = find_column_index(&headers, &["region", "area", "zone", "territory"]);
        let fiscal_year_idx = find_column_index(&headers, &["fiscal year", "year", "fy", "period"]);
        let project_number_idx = find_column_index(
            &headers,
            &["project number", "number", "id", "reference", "code"],
        );
        let framework_idx = find_column_index(
            &headers,
            &["framework", "method", "approach", "methodology"],
        );
        let project_profile_url_idx = find_column_index(
            &headers,
            &["project profile url", "url", "link", "website", "profile"],
        );

        for (i, row) in rows.enumerate() {
            // Helper to get string value from cell
            let get_string = |idx: Option<usize>| -> String {
                idx.and_then(|i| row.get(i))
                    .map(|cell| cell.to_string())
                    .unwrap_or_default()
            };

            // Helper to get float value from cell
            let get_float = |idx: Option<usize>| -> f64 {
                idx.and_then(|i| row.get(i))
                    .map(|cell| cell.to_string().parse().unwrap_or(0.0))
                    .unwrap_or(0.0)
            };

            let project_name = get_string(project_name_idx);
            if project_name.is_empty() {
                continue;
            }

            let project = Project {
                id: (i + 1) as f64,
                project_name,
                project_description: get_string(project_description_idx),
                country: get_string(country_idx),
                naics_sector: get_string(naics_sector_idx),
                committed: get_float(committed_idx),
                department: get_string(department_idx),
                project_type: get_string(project_type_idx),
                region: get_string(region_idx),
                fiscal_year: get_string(fiscal_year_idx),
                project_number: get_string(project_number_idx),
                framework: get_string(framework_idx),
                project_profile_url: get_string(project_profile_url_idx),
                tags: vec![],
                starred: false,
                comment: String::new(),
            };
            projects.push(project);
        }
    }

    let mappings = get_preference_to_filter_mappings();
    let mut recommended_projects = Vec::new();

    for project in projects {
        for preference in preferences {
            if let Some(mapping) = mappings.get(preference) {
                let naics_sectors = mapping
                    .get("naicsSectors")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .map(|s| s.as_str().unwrap().to_string())
                            .collect::<Vec<String>>()
                    })
                    .unwrap_or_default();
                let departments = mapping
                    .get("departments")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .map(|s| s.as_str().unwrap().to_string())
                            .collect::<Vec<String>>()
                    })
                    .unwrap_or_default();

                if (!naics_sectors.is_empty() && naics_sectors.contains(&project.naics_sector))
                    || (!departments.is_empty() && departments.contains(&project.department))
                {
                    recommended_projects.push(project.clone());
                    break; // Avoid duplicate projects
                }
            }
        }
    }

    // Limit the recommendations to 5 as mentioned in the commit
    recommended_projects.truncate(5);

    Ok(recommended_projects)
}
