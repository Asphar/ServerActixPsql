use easy_hasher::easy_hasher::*;
use std::io;
use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::Rng;



pub fn domain_salt(test: &str) -> String
{
	//let password: String = "test".to_string();
	//let owned_password: String = test.to_owned();
	let salt: &str = &"ShieldFactory".to_string();
	let salted_passwd: String = test.to_owned() + salt;
	//let salted_passwd: String = owned_password + salt;
	println!("\ndomain salt = {}\n\nsalted_passwd = '{}'", salt, salted_passwd);
	return salted_passwd;
}

pub fn sha_512(input_text: &str) -> String{
	/*
    let mut input_text = String::new();
    println!("Print your password : ");

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin"); 
	*/

    let salted_password: String = domain_salt(&input_text).to_string();

    println!("\nsalted password : {}\n", salted_password);

    let hash = sha512(&salted_password);
    let hashed_password = hash.to_hex_string();

    println!("SHA512({}) = {}\n\n Fin du SHA \n\n", salted_password, hashed_password);
    // return hashed_password;
    return hashed_password;
}
// SHA 512


pub fn private_salt() -> [u8; 32] {
	let salt: [u8; 32] = rand::thread_rng().gen();
	println!("Salt : {:?}\n", salt.as_ptr());
	return salt;
}

pub fn argon2(input_text: &str) -> String{
	let password = sha_512(&input_text);
	// Ici sera récupéré le mot de passe salé et haché en sha512 afin qu'il puisse être chiffré en argon2i 
	
	let salt = private_salt();
	let config = Config {
	    variant: Variant::Argon2d, 	// On peut choisir entre Argon2i, Argon2d et Argon2id
	    version: Version::Version13,
	    mem_cost: 65536,
	    time_cost: 10,			// Iterations
	    lanes: 4,				// Parallelism Factor
	    thread_mode: ThreadMode::Parallel,
	    secret: &[],
	    ad: &[],
	    hash_length: 32
	};
	// https://argon2.online/
	
	let argon_hash = argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();
	
	/*let matches = argon2::verify_encoded(&hash, password).unwrap();
	assert!(matches);*/
	
	println!("{:?}\n\n Fin de Argon2\n\n",argon_hash);
	return argon_hash;
}