export const idlFactory = ({ IDL }) => {
  const ToDo = IDL.Record({ 'completed' : IDL.Bool, 'description' : IDL.Text });
  return IDL.Service({
    'addTodo' : IDL.Func([IDL.Text], [IDL.Nat], ['query']),
    'clearCompleted' : IDL.Func([], [], []),
    'completeTodo' : IDL.Func([IDL.Nat], [], []),
    'getTodos' : IDL.Func([], [IDL.Vec(ToDo)], ['query']),
    'showTodos' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
