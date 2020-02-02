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
    println!("{}", jokenpo!("pedra", "papel"));
    println!("{}", jokenpo!("papel", "pedra"));

    println!("{}", jokenpo!("pedra", "tesoura"));
    println!("{}", jokenpo!("tesoura", "pedra"));

    println!("{}", jokenpo!("tesoura", "papel"));
    println!("{}", jokenpo!("papel", "tesoura"));

    println!("{}", jokenpo!("papel", "papel"));
    println!("{}", jokenpo!("tesoura", "tesoura"));
    println!("{}", jokenpo!("pedra", "pedra"));
}