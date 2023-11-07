export const idlFactory = ({ IDL }) => {
  const Denizen = IDL.Record({
    'xp' : IDL.Nat64,
    'principal' : IDL.Principal,
    'firstname' : IDL.Opt(IDL.Text),
    'level' : IDL.Nat64,
    'token_balance' : IDL.Nat64,
    'dname' : IDL.Text,
  });
  return IDL.Service({
    'create_denizen' : IDL.Func(
        [IDL.Text, IDL.Opt(IDL.Text), IDL.Nat64, IDL.Nat64, IDL.Nat64],
        [IDL.Opt(Denizen)],
        [],
      ),
    'get' : IDL.Func([IDL.Principal], [IDL.Opt(Denizen)], ['query']),
    'get_all_denizens' : IDL.Func([], [IDL.Vec(Denizen)], ['query']),
    'insert' : IDL.Func([IDL.Principal, Denizen], [IDL.Opt(Denizen)], []),
  });
};
export const init = ({ IDL }) => { return []; };
