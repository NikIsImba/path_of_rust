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
      className="bg-white absolute w-20 h-20 rounded-xl"
    >
      {`Group ${id}`}
    </div>
  )
}

export default Group;