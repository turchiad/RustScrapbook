fn main() {

	let coin1 = Coin::Quarter(UsState::Alabama);

	println!("coin1 is worth: {}", value_in_cents(coin1));
    
}

#[derive(Debug)]
enum UsState{
	Alabama,
	Alaska,
	American_Samoa,
	Arizona,
	Arkansas,
	California,
	Colorado,
	Connecticut,
	Delaware,
	District_of_Columbia,
	Florida,
	Georgia,
	Guam,
	Hawaii,
	Idaho,
	Illinois,
	Indiana,
	Iowa,
	Kansas,
	Kentucky,
	Louisiana,
	Maine,
	Maryland,
	Massachusetts,
	Michigan,
	Minnesota,
	Minor_Outlying_Islands,
	Mississippi,
	Missouri,
	Montana,
	Nebraska,
	Nevada,
	New_Hampshire,
	New_Jersey,
	New_Mexico,
	New_York,
	North_Carolina,
	North_Dakota,
	Northern_Mariana_Islands,
	Ohio,
	Oklahoma,
	Oregon,
	Pennsylvania,
	Puerto_Rico,
	Rhode_Island,
	South_Carolina,
	South_Dakota,
	Tennessee,
	Texas,
	US_Virgin_Islands,
	Utah,
	Vermont,
	Virginia,
	Washington,
	West_Virginia,
	Wisconsin,
	Wyoming
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}!",state);
			25
		}
	}
}
