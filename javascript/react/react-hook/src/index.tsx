import React, {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from 'react';
import ReactDOM from 'react-dom';
import { act } from 'react-dom/test-utils';

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

function Counter() {
  const [cnt, setCnt] = useState(0);
  const increaseCnt = () => setCnt((prev) => prev + 1);
  const decreaseCnt = () => setCnt((prev) => prev - 1);

  return (
    <>
      <h1>{cnt}</h1>
      <button onClick={increaseCnt}>+1</button>
      <button onClick={decreaseCnt}>-1</button>
    </>
  );
}

function SimpleInput() {
  const [text, setText] = useState('');

  const onChange = (e: React.ChangeEvent<any>) => {
    setText(e.target.value);
  };

  const onReset = () => {
    setText('');
  };

  return (
    <div>
      <input onChange={onChange} />
      <button onClick={onReset}>초기화</button>
      <div>
        <b>값: {text}</b>
      </div>
    </div>
  );
}

function MultiInputWithRef() {
  const initInputs = {
    name: '',
    nickname: '',
  };

  const [inputs, setInputs] = useState(initInputs);
  const nameInput = useRef<HTMLInputElement>(null);

  const onChange = (e: React.ChangeEvent<any>) => {
    const { name, value } = e.target;
    console.log(name, value);
    setInputs({
      ...inputs,
      [name]: value,
    });
  };
  const onReset = (e: React.ChangeEvent<any>) => {
    setInputs(initInputs);
    nameInput.current?.focus();
  };

  return (
    <div>
      <input
        name="name"
        onChange={onChange}
        ref={nameInput}
        placeholder="이름"
      />
      <input name="nickname" onChange={onChange} placeholder="닉네임" />
      <button onClick={onReset}>초기화</button>
      <div>이름: {inputs.name}</div>
      <div>닉네임: {inputs.nickname}</div>
    </div>
  );
}

interface UserProps {
  id: number;
  name: string;
  email: string;
  isActive: boolean;
  onRemove: (id: number) => void;
  onToggle: (id: number) => void;
}

function User(props: UserProps) {
  const { id, name, email, isActive, onToggle, onRemove } = props;

  useEffect(() => {
    console.log(`User ${id} is mounted`);
    return () => {
      console.log(`User ${id} is unmounted`);
    };
  }, [id]);

  return (
    <div>
      <button onClick={() => onRemove(id)}>삭제</button>
      <b
        style={{
          cursor: 'pointer',
          color: isActive ? 'green' : 'black',
        }}
        onClick={() => onToggle(id)}
      >
        <div>이름: {name}</div>
      </b>
      <div>이메일: {email}</div>
    </div>
  );
}

interface UserListProps {
  users: Omit<UserProps, 'onRemove' | 'onToggle'>[];
  onRemove: (id: number) => void;
  onToggle: (id: number) => void;
}

function UserList(props: UserListProps) {
  const { users, onRemove, onToggle } = props;

  useEffect(() => {
    console.log(`UserList is mounted`);
    return () => {
      console.log(`UserList is unmounted`);
    };
  }, [users]);

  return (
    <div>
      {users.map((user) => (
        <User key={user.id} onRemove={onRemove} onToggle={onToggle} {...user} />
      ))}
    </div>
  );
}
interface CreateUserProps {
  onUserCreate: (input: Pick<UserProps, 'name' | 'email'>) => void;
}

function CreateUser(props: CreateUserProps) {
  const [input, setInput] = useState({
    name: '',
    email: '',
  });

  const onChange = (e: React.ChangeEvent<any>) => {
    const { name, value } = e.target;

    setInput({
      ...input,
      [name]: value,
    });
  };

  const onCreate = () => {
    // add new user
    props.onUserCreate({ ...input });

    // refrest
    setInput({
      name: '',
      email: '',
    });
  };

  // const { name, email, onChange, onCreate } = props;
  return (
    <div>
      <input
        name="name"
        placeholder="name"
        onChange={onChange}
        value={input.name}
      />
      <input
        name="email"
        placeholder="email"
        onChange={onChange}
        value={input.email}
      />
      <button onClick={onCreate}>Submit</button>
    </div>
  );
}

function UserPage() {
  // UserList
  const [users, setUsers] = useState([
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
  ]);

  // const onRemove = useCallback(
  //   (id: number) => {
  //     setUsers(() => users.filter((user) => user.id !== id));
  //   },
  //   [users]
  // );
  const onRemove = useCallback((id: number) => {
    setUsers((users) => users.filter((user) => user.id !== id));
  }, []);
  const onToggle = useCallback((id: number) => {
    setUsers((users) =>
      users.map((user) =>
        user.id === id ? { ...user, isActive: !user.isActive } : user
      )
    );
  }, []);

  const userListProps = {
    users,
    onRemove,
    onToggle,
  };

  // UserCreate
  const onUserCreate = useCallback(
    (input: Pick<UserProps, 'name' | 'email'>) => {
      setUsers((users) => {
        const id = users.length ? users[users.length - 1].id + 1 : 0;
        const { name, email } = input;

        const user = {
          id,
          name,
          email,
          isActive: false,
        };
        return [...users, user];
      });
    },
    []
  );
  const createProps = {
    onUserCreate,
  };

  return (
    <>
      <CreateUser {...createProps} />
      <UserList {...userListProps} />
    </>
  );
}

ReactDOM.render(
  <React.StrictMode>
    <Wrapper>
      <Counter />
    </Wrapper>
    <Wrapper>
      <SimpleInput />
    </Wrapper>
    <Wrapper>
      <MultiInputWithRef />
    </Wrapper>
    <Wrapper>
      <UserPage />
    </Wrapper>
  </React.StrictMode>,
  document.getElementById('root')
);
