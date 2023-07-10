pub fn tab_to_string(tab: Vec<Vec<f32>>) -> String {
    tab
        .into_iter()
        .map(|row| row
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(" "))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn string_to_tab(string: String) -> Vec<Vec<f32>> {
    string
        .split('\n')
        .map(|row| row
            .split(' ')
            .map(|value| value.parse::<f32>().unwrap_or_else(|e| {
                eprintln!("ERROR!! Echec de la conversion d'une valeur de tableau en flottant : {e}");
                0.5
            }))
            .collect::<Vec<_>>())
        .collect::<Vec<Vec<f32>>>()
}

#[test]
fn test_serde() {
    let mut tab = Vec::new();
    for _ in 0..5 {
        let mut row = Vec::new();
        for i in 0..5 {
            row.push(i as f32 / 10.);
        }
        tab.push(row);
    }

    let serialized = tab_to_string(tab.clone());
    assert_eq!(serialized, "0 0.1 0.2 0.3 0.4\n0 0.1 0.2 0.3 0.4\n0 0.1 0.2 0.3 0.4\n0 0.1 0.2 0.3 0.4\n0 0.1 0.2 0.3 0.4");
    
    let deserialized = string_to_tab(serialized);
    assert_eq!(deserialized, tab);
}
