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
	const Result_1 = IDL.Variant({ Ok: IDL.Principal, Err: IDL.Null });
	const Dao = IDL.Record({
		id: IDL.Principal,
		controllers: IDL.Vec(IDL.Principal),
		owner: IDL.Principal,
		name: IDL.Text
	});
	const Result_2 = IDL.Variant({ Ok: IDL.Null, Err: IDL.Text });
	return IDL.Service({
		call_dao_function: IDL.Func([IDL.Text, IDL.Text], [Result], []),
		generate: IDL.Func([IDL.Text], [Result_1], []),
		get_daos: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Text, Dao))], ['query']),
		set_dao_name: IDL.Func([IDL.Text, IDL.Text], [], []),
		update_daos_code: IDL.Func([], [Result_2], [])
	});
};
export const init = ({ IDL }) => {
	return [];
};
