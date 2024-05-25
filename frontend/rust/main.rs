use crate::errors::UneError;
slint::include_modules!();

mod themes;
mod errors;


fn open_file(path: String) -> Result<String, &'static UneError<'static>> {
	let success = false;
	if success {
		return Ok(String::from( "Content" ));
	} else {
		Err(&errors::themes::IMAGE_NOT_FOUND_URL_MESSAGE)
	}
}

fn main() {
	let result = open_file(String::from("fdjsah"));
	match result
	{
		Ok(string) => println!("{}", string),
		Err(err) => {
			match err
			{
				&errors::themes::IMAGE_NOT_FOUND_URL_MESSAGE => println!("the message")
			}
		}
		//Err(&errors::themes::IMAGE_NOT_FOUND_URL_MESSAGE) => println!("error")
	}
}

// fn main()
// {
//
// 	let page = StartPage::new().unwrap();
// 	page.run().unwrap();
// }