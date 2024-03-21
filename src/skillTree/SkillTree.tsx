import React, { useState } from 'react';
import Group from './Group';
import Draggable from 'react-draggable';

interface SkillTreeProps {
  baseSize: number[];
  groupLocations: { [key: string]: [number, number] };
}


const SkillTree: React.FC<SkillTreeProps> = ({ baseSize, groupLocations }) => {
  const [scale, setScale] = useState(1);

  const handleWheel = (e: React.WheelEvent<HTMLDivElement>) => {
    e.preventDefault();
    const scaleChange = e.deltaY < 0 ? 1.1 : 0.9;
    setScale(prevScale => prevScale * scaleChange);
  }

  return (
    <div onWheel={handleWheel} style={{ transform: `scale(${scale})`, transformOrigin: '0 0' }}>
      <Draggable scale={scale}>
        <div
          className='bg-yellow-950 absolute'
          style={{ width: baseSize[0], height: baseSize[1] }}
        >
          {Object.entries(groupLocations).map(([key, value]) => (
            <Group key={key} id={key} position={value} />
          ))}
        </div>
      </Draggable>
    </div>
  )
}

export default SkillTree;