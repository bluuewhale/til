import React, { useState } from 'react';

type Validator = (...args: any[]) => boolean;

const useInput = (val: string, validator?: Validator) => {
  const [value, setValue] = useState(val);

  const onChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const {
      target: { value },
    } = event;

    let willUpdate = true;
    if (validator) {
      willUpdate = validator(value);
    }

    if (willUpdate) {
      setValue(value);
    } else {
      console.log('failed to update');
    }
  };

  return { value, onChange };
};

const maxLenValidator = (maxLen: number) => {
  return (val: string) => {
    return val.length < maxLen;
  };
};

function App() {
  const name = useInput('Mr.', maxLenValidator(10));
  return (
    <div className="UseInput">
      <h1>[UseInput]</h1>
      <input {...name}></input>
    </div>
  );
}

export default App;
