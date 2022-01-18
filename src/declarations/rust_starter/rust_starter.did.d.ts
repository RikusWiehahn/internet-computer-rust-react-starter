import type { Principal } from '@dfinity/principal';
export interface Note { 'id' : string, 'title' : string, 'contents' : string }
export interface Profile {
  'id' : string,
  'name' : string,
  'description' : string,
  'keywords' : Array<string>,
}
export interface UpdateProfile {
  'name' : string,
  'description' : string,
  'keywords' : Array<string>,
}
export interface _SERVICE {
  'create' : (arg_0: UpdateProfile) => Promise<[] | [Profile]>,
  'createNote' : (arg_0: Note) => Promise<[] | [Note]>,
  'deleteNote' : (arg_0: string) => Promise<boolean>,
  'findUnique' : (arg_0: string) => Promise<[] | [Profile]>,
  'getAllNotes' : () => Promise<Array<Note>>,
  'getSelf' : () => Promise<[] | [Profile]>,
  'search' : (arg_0: string) => Promise<[] | [Profile]>,
  'update' : (arg_0: UpdateProfile) => Promise<[] | [Profile]>,
  'updateNote' : (arg_0: Note) => Promise<[] | [Note]>,
}
