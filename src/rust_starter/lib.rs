use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::vec::Vec;

mod types;
use types::*;

//
//   ####  #####   ##   ##### ######
//  #        #    #  #    #   #
//   ####    #   #    #   #   #####
//       #   #   ######   #   #
//  #    #   #   #    #   #   #
//   ####    #   #    #   #   ######

type ProfileList = Vec<Profile>;
type NoteList = Vec<Note>;

//
//  ###### #    # #    #  ####  ##### #  ####  #    #  ####
//  #      #    # ##   # #    #   #   # #    # ##   # #
//  #####  #    # # #  # #        #   # #    # # #  #  ####
//  #      #    # #  # # #        #   # #    # #  # #      #
//  #      #    # #   ## #    #   #   # #    # #   ## #    #
//  #       ####  #    #  ####    #   #  ####  #    #  ####

#[query(name = "getSelf")]
async fn get_self() -> Option<Profile> {
	let id = ic_cdk::caller().to_text();
	let profile_list = storage::get::<ProfileList>();
	if is_anonymous(&id).await {
		return None;
	}

	for profile in profile_list.clone() {
		if profile.id == id {
			return Some(profile);
		}
	}
	return None;
}

#[query(name = "findUnique")]
async fn find_unique(id: String) -> Option<Profile> {
	let profile_list = storage::get::<ProfileList>();
	if is_anonymous(&id).await {
		return None;
	}

	for profile in profile_list.clone() {
		if profile.id == id {
			return Some(profile);
		}
	}
	return None;
}

#[update]
async fn create(profile: UpdateProfile) -> Option<Profile> {
	let id = ic_cdk::caller().to_text();
	let profile_list = storage::get_mut::<ProfileList>();
	let mut exists = false;
	if is_anonymous(&id).await {
		return None;
	}

	for p in profile_list.clone() {
		if p.id == id {
			exists = true;
			break;
		}
	}
	if exists == false {
		let profile = Profile {
			id,
			name: profile.name,
			description: profile.description,
			keywords: profile.keywords,
		};
		profile_list.push(profile.clone());
		return Some(profile.clone());
	}
	return None;
}

#[update]
async fn update(profile: UpdateProfile) -> Option<Profile> {
	let id = ic_cdk::caller().to_text();
	let profile_list = storage::get_mut::<ProfileList>();
	if is_anonymous(&id).await {
		return None;
	}

	for i in 0..profile_list.len() {
		if profile_list[i].id == id.clone() {
			profile_list[i] = Profile {
				id: id.clone(),
				name: profile.name.clone(),
				description: profile.description.clone(),
				keywords: profile.keywords.clone(),
			};
			return Some(profile_list[i].clone());
		}
	}
	return None;
}

#[query]
async fn search(text: String) -> Option<Profile> {
	let text = text.to_lowercase();
	let profile_list = storage::get_mut::<ProfileList>();

	for profile in profile_list.clone() {
		if profile.name.to_lowercase().contains(&text)
			|| profile.description.to_lowercase().contains(&text)
		{
			return Some(profile);
		}
		for x in profile.keywords.iter() {
			if x.to_lowercase() == text {
				return Some(profile);
			}
		}
	}
	return None;
}

#[cfg(feature = "v4")]
#[update(name = "createNote")]
async fn create_note(note: Note) -> Option<Note> {
	let note_list = storage::get_mut::<NoteList>();
	let new_note_id: String = Uuid::new_v4().to_string();

	let note = Note {
		id: new_note_id.clone(),
		title: note.title.clone(),
		contents: note.contents.clone(),
	};
	note_list.push(note.clone());
	return Some(note.clone());
}

#[query(name = "getAllNotes")]
async fn get_all_notes() -> Vec<Note> {
	let note_list = storage::get::<NoteList>();
	return note_list.clone();
}

#[update(name = "updateNote")]
async fn update_note(note: Note) -> Option<Note> {
	let note_list = storage::get_mut::<NoteList>();

	for i in 0..note_list.len() {
		if note_list[i].id == note.id.clone() {
			note_list[i] = note.clone();
			return Some(note.clone());
		}
	}
	return None;
}

#[update(name = "deleteNote")]
async fn delete_note(id: String) -> bool {
	let note_list = storage::get_mut::<NoteList>();

	for i in 0..note_list.len() {
		if note_list[i].id == id {
			note_list.remove(i);
			return true;
		}
	}
	return false;
}

//
//  #    # ##### # #       ####
//  #    #   #   # #      #
//  #    #   #   # #       ####
//  #    #   #   # #           #
//  #    #   #   # #      #    #
//   ####    #   # ######  ####

async fn is_anonymous(id: &str) -> bool {
	if id == "2vxsx-fae" {
		return true;
	}
	return false;
}


