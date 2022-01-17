export const idlFactory = ({ IDL }) => {
  const UpdateProfile = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'keywords' : IDL.Vec(IDL.Text),
  });
  const Profile = IDL.Record({
    'id' : IDL.Text,
    'name' : IDL.Text,
    'description' : IDL.Text,
    'keywords' : IDL.Vec(IDL.Text),
  });
  const Note = IDL.Record({
    'id' : IDL.Text,
    'title' : IDL.Text,
    'contents' : IDL.Text,
  });
  return IDL.Service({
    'create' : IDL.Func([UpdateProfile], [IDL.Opt(Profile)], []),
    'createNote' : IDL.Func([Note], [IDL.Opt(Note)], []),
    'deleteNote' : IDL.Func([IDL.Text], [IDL.Bool], []),
    'findUnique' : IDL.Func([IDL.Text], [IDL.Opt(Profile)], ['query']),
    'getAllNotes' : IDL.Func([], [IDL.Vec(Note)], ['query']),
    'getSelf' : IDL.Func([], [IDL.Opt(Profile)], ['query']),
    'search' : IDL.Func([IDL.Text], [IDL.Opt(Profile)], ['query']),
    'update' : IDL.Func([UpdateProfile], [IDL.Opt(Profile)], []),
    'updateNote' : IDL.Func([Note], [IDL.Opt(Note)], []),
  });
};
export const init = ({ IDL }) => { return []; };
