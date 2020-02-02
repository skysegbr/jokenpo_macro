macro_rules! jokenpo {
    ($param_l: expr, $param_r: expr) => {
        if ($param_l == "pedra" &&  $param_r == "papel") || ($param_l == "papel" &&  $param_r == "pedra") {
            "papel"
        } else if ($param_l == "pedra" &&  $param_r == "tesoura") || ($param_l == "tesoura" &&  $param_r == "pedra") {
            "pedra"
        } else if ($param_l == "papel" &&  $param_r == "tesoura") || ($param_l == "tesoura" &&  $param_r == "papel") {
            "tesoura"
        }else {
            "empate"
        }
    };
}

macro_rules! jokenpo2 {
    ($param_l: expr, $param_r: expr) => {{
        let p_l = $param_l; 
        let p_r = $param_r; 
        if (p_l == "pedra" &&  p_r == "papel") || (p_l == "papel" &&  p_r == "pedra") {
            "papel"
        } else if (p_l == "pedra" &&  p_r == "tesoura") || (p_l == "tesoura" &&  p_r == "pedra") {
            "pedra"
        } else if (p_l == "papel" &&  p_r == "tesoura") || (p_l == "tesoura" &&  p_r == "papel") {
            "tesoura"
        }else {
            "empate"
        }
    }};
}

macro_rules! jokenpo3 {
    ($param_l: expr, $param_r: expr) => {
       match($param_l, $param_r){
        ("pedra",  "papel") | ("papel", "pedra")=> "papel",
        ("papel",  "tesoura") | ("tesoura",  "papel") => "tesoura",
        ("pedra",  "tesoura") | ("tesoura",  "pedra") => "pedra",
        (_, _) => "empatou"
       }
    };
}

#[test]
fn teste_jokenpo3(){
    assert_eq!(jokenpo3!("pedra", "papel"), "papel");
    assert_eq!(jokenpo3!("papel", "pedra"), "papel");
    
    assert_eq!(jokenpo3!("pedra", "tesoura"), "pedra");
    assert_eq!(jokenpo3!("tesoura", "pedra"), "pedra");
    
    assert_eq!(jokenpo3!("tesoura", "papel"), "tesoura");
    assert_eq!(jokenpo3!("papel", "tesoura"), "tesoura");
    
    assert_eq!(jokenpo3!("papel", "papel"), "empatou");
    assert_eq!(jokenpo3!("tesoura", "tesoura"), "empatou");
    assert_eq!(jokenpo3!("pedra", "pedra"), "empatou");

}

#[test]
fn teste_jokenpo2(){
    assert_eq!(jokenpo2!("pedra", "papel"), "papel");
    assert_eq!(jokenpo2!("papel", "pedra"), "papel");
    
    assert_eq!(jokenpo2!("pedra", "tesoura"), "pedra");
    assert_eq!(jokenpo2!("tesoura", "pedra"), "pedra");
    
    assert_eq!(jokenpo2!("tesoura", "papel"), "tesoura");
    assert_eq!(jokenpo2!("papel", "tesoura"), "tesoura");
    
    assert_eq!(jokenpo2!("papel", "papel"), "empate");
    assert_eq!(jokenpo2!("tesoura", "tesoura"), "empate");
    assert_eq!(jokenpo2!("pedra", "pedra"), "empate");

}

#[test]
fn teste_jokenpo(){
    assert_eq!(jokenpo!("pedra", "papel"), "papel");
    assert_eq!(jokenpo!("papel", "pedra"), "papel");
    
    assert_eq!(jokenpo!("pedra", "tesoura"), "pedra");
    assert_eq!(jokenpo!("tesoura", "pedra"), "pedra");
    
    assert_eq!(jokenpo!("tesoura", "papel"), "tesoura");
    assert_eq!(jokenpo!("papel", "tesoura"), "tesoura");
    
    assert_eq!(jokenpo!("papel", "papel"), "empate");
    assert_eq!(jokenpo!("tesoura", "tesoura"), "empate");
    assert_eq!(jokenpo!("pedra", "pedra"), "empate");
}

fn main(){

    //Jokenpo
    println!("{}", jokenpo!("pedra", "papel"));
    println!("{}", jokenpo!("papel", "pedra"));

    println!("{}", jokenpo!("pedra", "tesoura"));
    println!("{}", jokenpo!("tesoura", "pedra"));

    println!("{}", jokenpo!("tesoura", "papel"));
    println!("{}", jokenpo!("papel", "tesoura"));

    println!("{}", jokenpo!("papel", "papel"));
    println!("{}", jokenpo!("tesoura", "tesoura"));
    println!("{}", jokenpo!("pedra", "pedra"));

    //Jokenpo2 
    println!("{}", jokenpo2!("pedra", "papel"));
    println!("{}", jokenpo2!("papel", "pedra"));

    println!("{}", jokenpo2!("pedra", "tesoura"));
    println!("{}", jokenpo2!("tesoura", "pedra"));

    println!("{}", jokenpo2!("tesoura", "papel"));
    println!("{}", jokenpo2!("papel", "tesoura"));

    println!("{}", jokenpo2!("papel", "papel"));
    println!("{}", jokenpo2!("tesoura", "tesoura"));
    println!("{}", jokenpo2!("pedra", "pedra"));

    //Jokenpo3
    println!("{}", jokenpo3!("pedra", "papel"));
    println!("{}", jokenpo3!("papel", "pedra"));

    println!("{}", jokenpo3!("pedra", "tesoura"));
    println!("{}", jokenpo3!("tesoura", "pedra"));

    println!("{}", jokenpo3!("tesoura", "papel"));
    println!("{}", jokenpo3!("papel", "tesoura"));

    println!("{}", jokenpo3!("papel", "papel"));
    println!("{}", jokenpo3!("tesoura", "tesoura"));
    println!("{}", jokenpo3!("pedra", "pedra"));
}