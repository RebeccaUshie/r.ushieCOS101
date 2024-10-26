fn main() {
	let ta:f64 = 450_000.00;
	let tq:f64 = 2.0;
	let ma:f64 = 1_500_000.00;
	let mq:f64 = 1.0;
	let ha:f64 = 750_000.00;
	let hq:f64 = 3.0;
	let da:f64 = 2_850_000.00;
	let dq:f64 = 3.0;
	let aa:f64 = 250_000.00;
	let aq:f64 = 1.0;

	//Toshiba
	let t = ta*tq;
	println!("Toshiba is {}",t );

	//Mac
	let m = ma*mq; 
	println!("Mac is {}",m );

	//HP
	let h = ha*hq;
	println!("HP is {}",h );

	//Dell 
	let d = da*dq;
	println!("Dell is {}",d );

	//Acer
	let a = aa*aq;
	println!("Acer is {}",a );

	//Total quantity
	let q = tq + mq + hq + dq + aq;
	println!("Total quantity is {}",q );

	//Sum of sales record
	let sum = ta + ma + ha + da + aa;
	println!("Sum is {}",sum );

	//Average
	let average = sum / q;
	println!("Average is {}",average );
}