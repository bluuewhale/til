import React, { useEffect, useState } from 'react';

const useTitle = (_title: string) => {
  const [title, setTitle] = useState(_title);

  const updateTitle = () => {
    const htmlTitle = document.querySelector('title');
    if (htmlTitle) htmlTitle.innerText = title;
  };

  useEffect(updateTitle, [title]);

  return { setTitle };
};
export default function UseEffect() {
  const { setTitle } = useTitle('Loading...');
  setTimeout(() => setTitle('Home'), 1000);

  return <div></div>;
}
