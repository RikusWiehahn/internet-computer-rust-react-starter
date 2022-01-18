use ic_cdk::export::candid::{CandidType, Deserialize};

//
//  ##### #   # #####  ######  ####
//    #    # #  #    # #      #
//    #     #   #    # #####   ####
//    #     #   #####  #           #
//    #     #   #      #      #    #
//    #     #   #      ######  ####

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Profile {
	pub id: String,
	pub name: String,
	pub description: String,
	pub keywords: Vec<String>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct UpdateProfile {
	pub name: String,
	pub description: String,
	pub keywords: Vec<String>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Note {
	pub id: String,
	pub title: String,
	pub contents: String,
}