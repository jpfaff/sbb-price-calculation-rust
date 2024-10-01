enum AboSettings {
    Ga,
    ZvvAll,
    Halbtax,
}

impl AboSettings {
    fn as_str(&self) -> &'static str {
        match self {
            AboSettings::Ga => "GA",
            AboSettings::ZvvAll => "ZVV Alle Zonen",
            AboSettings::Halbtax => "Halbtax",
        }
    }

    fn single_cost_function(&self, r: &Route) -> f32 {
        match self {
            AboSettings::Ga => 0.0,
            AboSettings::ZvvAll => 0.0,
            AboSettings::Halbtax => r.single_cost / 2.0,
        }
    }
}

struct AboType {
    name: AboSettings,
    monthly_cost: Option<f32>,
    annual_cost: Option<f32>,
}

impl AboType {
    fn get_single_cost(&self, r: &Route) -> f32 {
        self.name.single_cost_function(r)
    }

    fn get_monthly_cost(&self) -> f32 {
        if let Some(x) = self.monthly_cost {
            x
        } 
        else {
            0.0
        }
    }

    fn get_annual_cost(&self) -> f32 {
        if let Some(x) = self.annual_cost {
            x
        } 
        else {
            0.0
        }
    }
}

struct Route {
    name: String,
    start: String,
    end: String,
    viable_abos: Vec<AboType>,
    single_cost: f32,
}


fn main() {
    let a = AboType {
        name: AboSettings::ZvvAll,
        monthly_cost: Some(70.0),
        annual_cost: Some(100.0),
    };
    let b = AboType {
        name: AboSettings::Halbtax,
        monthly_cost: Some(120.0),
        annual_cost: Some(100.0),
    };
    println!("Abo name: {}", a.name.as_str());
    if let Some(x) = a.annual_cost {
        println!("Costs {} annualy", x);
    }
    if let Some(x) = a.monthly_cost {
        println!("Costs {} monthly", x);
    }

    let work_route = Route {
        name: String::from("Work commute"),
        start: String::from("Uster"),
        end: String::from("Horgen"),
        viable_abos: vec![a, b],
        single_cost: 15.60,
    };

    let current_abo = work_route.viable_abos.get(1).unwrap();
    println!("{} costs {} with {}", work_route.name, current_abo.get_single_cost(&work_route), current_abo.name.as_str());
    println!("plus {} monthly or {} yearly", current_abo.get_monthly_cost(), current_abo.get_annual_cost())

}
