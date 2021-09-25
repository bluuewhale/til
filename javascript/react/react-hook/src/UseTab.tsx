import React, { useState } from 'react';

const content = [
  {
    tab: 'Section 1',
    content: "I'm section 1",
  },
  {
    tab: 'Section 2',
    content: "I'm section 2",
  },
];

const useTabs = (idx: number, contents: any[]) => {
  const [currentIdx, setCurrentIdx] = useState(idx);

  return {
    currentItem: contents[currentIdx],
    changeItem: setCurrentIdx,
  };
};

export default function Tabs() {
  const tabs = useTabs(0, content);

  return (
    <div className="UseTab">
      <h1>[UseTab]</h1>
      <div id="section-buttons">
        {content.map((section, idx) => (
          <button key={idx} onClick={() => tabs.changeItem(idx)}>
            {section.tab}
          </button>
        ))}
      </div>
      <div>{tabs.currentItem.content}</div>
    </div>
  );
}
