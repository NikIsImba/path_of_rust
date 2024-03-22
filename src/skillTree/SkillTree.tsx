import React, { useEffect, useRef, useState } from 'react';
import Group from './Group';
import Draggable from 'react-draggable';

interface SkillTreeProps {
  baseSize: number[];
  groupLocations: { [key: string]: [number, number] };
}


const SkillTree: React.FC<SkillTreeProps> = ({ baseSize, groupLocations }) => {
  const [scale, setScale] = useState(1);
  const divRef = useRef<HTMLDivElement>(null);
  const draggableRef = React.useRef(null);

  const handleWheel = (e: WheelEvent) => {
    e.preventDefault();
    const scaleChange = e.deltaY < 0 ? 1.1 : 0.9;
    setScale(prevScale => prevScale * scaleChange);
  }

  useEffect(() => {
    const div = divRef.current;
    if (div) {
      div.addEventListener('wheel', handleWheel, { passive: false });
      return () => {
        div.removeEventListener('wheel', handleWheel);
      };
    }
  }, []);

  return (
    <div ref={divRef} style={{ transform: `scale(${scale})`, transformOrigin: '0 0' }}>
      <Draggable scale={scale} nodeRef={draggableRef}>
        <div
          ref={draggableRef}
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