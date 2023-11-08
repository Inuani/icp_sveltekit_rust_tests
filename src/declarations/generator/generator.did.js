export const idlFactory = ({ IDL }) => {
	return IDL.Service({ generate: IDL.Func([IDL.Text], [], []) });
};
export const init = ({ IDL }) => {
	return [];
};
