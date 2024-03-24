import React from 'react';

interface GroupProps {
  id: string;
  position: [number, number];
}

const Group: React.FC<GroupProps> = ({ id, position }) => {
  return (
    <div
      key={`Group ${id}`}
      style={{ bottom: `${position[1]}px`, left: `${position[0]}px` }}
      className="absolute h-20  w-20 rounded-xl bg-white"
    >
      {`Group ${id}`}
    </div>
  );
};

export default Group;
