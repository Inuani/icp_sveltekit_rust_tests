export const idlFactory = ({ IDL }) => {
	const SegmentArgs = IDL.Record({
		controllers: IDL.Vec(IDL.Principal),
		name: IDL.Text
	});
	return IDL.Service({
		get_name: IDL.Func([], [IDL.Text], ['query']),
		set_name: IDL.Func([IDL.Text], [], [])
	});
};
export const init = ({ IDL }) => {
	const SegmentArgs = IDL.Record({
		controllers: IDL.Vec(IDL.Principal),
		name: IDL.Text
	});
	return [SegmentArgs];
};
