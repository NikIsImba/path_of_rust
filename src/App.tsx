import { useEffect, useState } from 'react';
import { invokeAndSet } from './util/rustUtility';
import SkillTree from './skillTree/SkillTree';

function App() {
  const [baseSize, setBaseSize] = useState([0, 0]);
  const [groupLocations, setGroupLocations] = useState<{ [key: string]: [number, number] }>({});


  useEffect(() => {
    document.body.style.overflow = 'hidden';
    invokeAndSet(setBaseSize, 'get_base_size');
    invokeAndSet(setGroupLocations, 'get_group_locations');

    return () => {
      document.body.style.overflow = 'auto';
    }
  }, []);

  if (baseSize[0] === 0 || baseSize[1] === 0 || Object.keys(groupLocations).length === 0) return (<div>Loading...</div>);

  return (
    <div className='bg-gray-800'>
      <SkillTree baseSize={baseSize} groupLocations={groupLocations} />
    </div>
  )
}

export default App