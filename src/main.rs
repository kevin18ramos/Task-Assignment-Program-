use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;
use rand::Rng;


fn main() {




    // the groups being grabbed
    const GROUPS :[& str;4] = [
        "A",
        "B",
        "C",
        "D"
    ];

    // for loop for the file group names
    for group_specific in GROUPS{
        
        let file_name = "GROUP".to_owned() + {group_specific};
        println!("");
        println!("GROUP {group_specific}");

        // Group B being 5 people, requires a different list field
            
            const GROUPA :[& str;6] = ["Errin","Luke","Gabriel","Antonio","Malcolm","Cooper"];

            const GROUPB: [& str; 5] = ["Chevy","Angela","Colt","Amber","Kevin"];

            const GROUPC: [& str; 6] = ["Brooks","River","Isaiah","Joshua","Mike","Zaul"];
        
            const GROUPD: [& str; 6] = ["Lukas","Dylan (Eric)","Dylan","Cannon","Anna","JW"];


            

            

            let mut input = vec!["dishes/microwaves/Wipe Coffee area/Remove Coffee Filter", "trash/ turn on washing machine","sweeping and in kitchen", 
            "wipe tables/kitchen tables","sweeping and in bathrooms"];

            let mut input_6 = vec![
                "dishes/microwaves/Wipe Coffee area/Remove Coffee Filter", "trash/ turn on washing machine",
                "sweeping and in kitchen", "wipe tables/kitchen tables","sweeping and in bathrooms", "wipe tables/furniture"];

            let mut irs = Irs::default();
            let mut rng = StepRng::new(2, 13);
            irs.shuffle(&mut input, &mut rng);
            irs.shuffle(&mut input_6, &mut rng);

            let mut index = 0;

            // if group "filename" is group B
            if file_name == "GROUPA"{
                let numberone = rand::thread_rng().gen_range(1..=5);
                let numbertwo = rand::thread_rng().gen_range(1..=20);
                let mut rng = StepRng::new(numberone, numbertwo);
                irs.shuffle(&mut input_6, &mut rng);
                for y in input_6{
                    let name_of_person = GROUPA[index];
                    println!("{name_of_person} - {y}");
                    index += 1;
                };
            }
            else if file_name == "GROUPB"{
                let numberone = rand::thread_rng().gen_range(1..=5);
                let numbertwo = rand::thread_rng().gen_range(1..=20);
                let mut rng = StepRng::new(numberone, numbertwo);
                irs.shuffle(&mut input, &mut rng);
                for y in input{
                    let name_of_person = GROUPB[index];
                    println!("{name_of_person} - {y}");
                    index += 1;
                };
            }
            else if file_name == "GROUPC"{
                let numberone = rand::thread_rng().gen_range(1..=5);
                let numbertwo = rand::thread_rng().gen_range(1..=20);
                let mut rng = StepRng::new(numberone, numbertwo);
                irs.shuffle(&mut input_6, &mut rng);
                for y in input_6{
                    let name_of_person = GROUPC[index];
                    println!("{name_of_person} - {y}");
                    index += 1;
                };
            }
            else if file_name == "GROUPD"{
                let numberone = rand::thread_rng().gen_range(1..=5);
                let numbertwo = rand::thread_rng().gen_range(1..=20);
                let mut rng = StepRng::new(numberone, numbertwo);
                irs.shuffle(&mut input_6, &mut rng);
                for y in input_6{
                    let name_of_person = GROUPD[index];
                    println!("{name_of_person} - {y}");
                    index += 1;
                };
            }

    };    
}




