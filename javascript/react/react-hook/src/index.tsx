import React, { useCallback, useContext, useEffect, useReducer } from 'react';
import ReactDOM from 'react-dom';

interface AppProp {
  name: string;
  color: string;
  isSpecial: boolean;
}

function App(props: AppProp) {
  const { name, color, isSpecial } = props;
  return (
    <div style={{ color }}>
      {isSpecial && <b>*</b>}
      Hello {name}
    </div>
  );
}

App.defaultProps = {
  name: 'World!',
  color: 'red',
  isSpecial: false,
};

interface WrapperProps {
  children: JSX.Element | JSX.Element[];
}

function Wrapper(elem: WrapperProps) {
  const { children } = elem;
  const style = {
    border: '2px solid black',
    padding: '16px',
  };

  return <div style={style}>{children}</div>;
}

type CountState = number;
type CountAction = { type: CountActionType };
type CountActionType = 'INCREMENT' | 'DECREMENT';
function countReducer(state: CountState, action: CountAction) {
  switch (action.type) {
    case 'INCREMENT':
      return state + 1;
    case 'DECREMENT':
      return state - 1;
  }
}

function Counter() {
  const [cnt, dispatch] = useReducer(countReducer, 0);

  const increaseCnt = () => dispatch({ type: 'INCREMENT' });
  const decreaseCnt = () => dispatch({ type: 'DECREMENT' });

  return (
    <>
      <h1>{cnt}</h1>
      <button onClick={increaseCnt}>+1</button>
      <button onClick={decreaseCnt}>-1</button>
    </>
  );
}

// Reducer

type UserState = {
  inputs: Omit<UserCreateProps, 'onChange' | 'onCreate'>;
  users: Omit<UserProps, 'onRemove' | 'onToggle'>[];
};

type UserAction =
  | { type: 'CHANGE_INPUT'; name: string; value: string }
  | { type: 'CREATE_USER'; name: string; email: string }
  | { type: 'REMOVE_USER'; id: number }
  | { type: 'TOGGLE_USER'; id: number };

const initialState: UserState = {
  inputs: {
    name: '',
    email: '',
  },
  users: [
    {
      id: 0,
      name: 'foo',
      email: 'foo@gmail.com',
      isActive: true,
    },
    {
      id: 1,
      name: 'bar',
      email: 'bar@gmail.com',
      isActive: false,
    },
  ],
};

const UserStateContext = React.createContext<UserState>(initialState);
const UserDispatchContext = React.createContext<React.Dispatch<UserAction>>(
  () => null
);

function userReducer(state: UserState, action: UserAction): UserState {
  switch (action.type) {
    case 'CHANGE_INPUT': {
      return {
        ...state,
        inputs: {
          ...state.inputs,
          [action.name]: action.value,
        },
      };
    }
    case 'CREATE_USER': {
      const users = state.users;
      const nextId = users.length ? users[users.length - 1].id + 1 : 0;

      return {
        ...state,
        inputs: {
          name: '',
          email: '',
        },
        users: [
          ...users,
          {
            id: nextId,
            name: action.name,
            email: action.email,
            isActive: false,
          },
        ],
      };
    }
    case 'REMOVE_USER': {
      return {
        ...state,
        users: state.users.filter((user) => user.id !== action.id),
      };
    }
    case 'TOGGLE_USER': {
      return {
        ...state,
        users: state.users.map((user) => {
          if (user.id !== action.id) {
            return user;
          }
          return {
            ...user,
            isActive: !user.isActive,
          };
        }),
      };
    }
  }
}

type UserProps = {
  id: number;
  name: string;
  email: string;
  isActive: boolean;
};

function User(props: UserProps) {
  const { id, name, email, isActive } = props;
  const dispatch = useContext(UserDispatchContext);

  useEffect(() => {
    console.log(`User ${id} is mounted`);
    return () => {
      console.log(`User ${id} is unmounted`);
    };
  }, [id]);

  const onRemove = useCallback(
    () =>
      dispatch({
        type: 'REMOVE_USER',
        id: id,
      }),
    [id]
  );

  const onToggle = useCallback(
    () =>
      dispatch({
        type: 'TOGGLE_USER',
        id: id,
      }),
    [id]
  );

  return (
    <div>
      <button onClick={onRemove}>삭제</button>
      <b
        style={{
          cursor: 'pointer',
          color: isActive ? 'green' : 'black',
        }}
        onClick={onToggle}
      >
        <div>이름: {name}</div>
      </b>
      <div>이메일: {email}</div>
    </div>
  );
}

interface UserListProps {
  users: Omit<UserProps, 'onRemove' | 'onToggle'>[];
}

function UserList(props: UserListProps) {
  const { users } = props;

  useEffect(() => {
    console.log(`UserList is mounted`);
    return () => {
      console.log(`UserList is unmounted`);
    };
  }, [users]);

  return (
    <div>
      {users.map((user) => (
        <User key={user.id} {...user} />
      ))}
    </div>
  );
}

type UserCreateProps = {
  name: string;
  email: string;
};
function CreateUser(props: UserCreateProps) {
  const { name, email } = props;
  const dispatch = useContext(UserDispatchContext);

  const onChange = useCallback(
    (e: React.ChangeEvent<any>) => {
      const { name, value } = e.target;
      dispatch({
        name,
        value,
        type: 'CHANGE_INPUT',
      });
    },
    [dispatch]
  );

  const onCreate = useCallback(() => {
    dispatch({
      name,
      email,
      type: 'CREATE_USER',
    });
  }, [name, email, dispatch]);

  return (
    <div>
      <input name="name" placeholder="name" onChange={onChange} value={name} />
      <input
        name="email"
        placeholder="email"
        onChange={onChange}
        value={email}
      />
      <button onClick={onCreate}>Submit</button>
    </div>
  );
}

function UserPage() {
  // UserList
  const [state, dispatch] = useReducer(userReducer, initialState);

  const userListProps = {
    users: state.users,
  };

  // UserCreate
  const createUserProps = {
    name: state.inputs.name,
    email: state.inputs.email,
  };

  return (
    <UserStateContext.Provider value={state}>
      <UserDispatchContext.Provider value={dispatch}>
        <CreateUser {...createUserProps} />
        <UserList {...userListProps} />
      </UserDispatchContext.Provider>
    </UserStateContext.Provider>
  );
}

ReactDOM.render(
  <React.StrictMode>
    <Wrapper>
      <Counter />
    </Wrapper>
    <Wrapper>
      <UserPage />
    </Wrapper>
  </React.StrictMode>,
  document.getElementById('root')
);
