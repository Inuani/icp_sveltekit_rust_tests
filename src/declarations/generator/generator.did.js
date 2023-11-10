export const idlFactory = ({ IDL }) => {
	const RejectionCode = IDL.Variant({
		NoError: IDL.Null,
		CanisterError: IDL.Null,
		SysTransient: IDL.Null,
		DestinationInvalid: IDL.Null,
		Unknown: IDL.Null,
		SysFatal: IDL.Null,
		CanisterReject: IDL.Null
	});
	const Result = IDL.Variant({
		Ok: IDL.Tuple(IDL.Text),
		Err: IDL.Tuple(RejectionCode, IDL.Text)
	});
	const SocietyArgs = IDL.Record({
		name: IDL.Text,
		description: IDL.Text
	});
	const Result_1 = IDL.Variant({ Ok: IDL.Principal, Err: IDL.Text });
	const SocietyInfos = IDL.Record({
		is_denisen: IDL.Bool,
		name: IDL.Text,
		canister_id: IDL.Principal,
		description: IDL.Text,
		a_denizens: IDL.Nat64
	});
	const Society = IDL.Record({
		denizens_principals: IDL.Vec(IDL.Principal),
		controllers: IDL.Vec(IDL.Principal),
		owner: IDL.Principal,
		name: IDL.Text,
		canister_id: IDL.Principal,
		description: IDL.Text
	});
	const Result_2 = IDL.Variant({ Ok: Society, Err: IDL.Text });
	const Result_3 = IDL.Variant({
		Ok: IDL.Null,
		Err: IDL.Tuple(RejectionCode, IDL.Text)
	});
	const Result_4 = IDL.Variant({ Ok: IDL.Null, Err: IDL.Text });
	return IDL.Service({
		call_society_function: IDL.Func([IDL.Text, IDL.Text], [Result], []),
		create_society: IDL.Func([SocietyArgs], [Result_1], []),
		get_my_societies: IDL.Func([], [IDL.Vec(SocietyInfos)], ['query']),
		get_societies: IDL.Func([], [IDL.Vec(SocietyInfos)], ['query']),
		get_society: IDL.Func([IDL.Text], [Result_2], ['query']),
		set_society_name: IDL.Func([IDL.Text, IDL.Text], [Result_3], []),
		update_societies_wasm_code: IDL.Func([], [Result_3], []),
		update_society: IDL.Func([IDL.Text, SocietyArgs], [Result_4], [])
	});
};
export const init = ({ IDL }) => {
	return [];
};
