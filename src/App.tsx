import { useEffect, useState } from 'react';
import { invokeAndSet } from './util/rustUtility';
import SkillTree, { TsBaseSize } from './skillTree/SkillTree';
import { TsGroupLocation } from './skillTree/Group';

function App() {
  const [baseSize, setBaseSize] = useState<TsBaseSize | null>(null);
  const [groupLocations, setGroupLocations] = useState<
    TsGroupLocation[] | null
  >(null);

  useEffect(() => {
    document.body.style.overflow = 'hidden';
    invokeAndSet(setBaseSize, 'get_base_size');
    invokeAndSet(setGroupLocations, 'get_group_locations');

    return () => {
      document.body.style.overflow = 'auto';
    };
  }, []);

  if (baseSize == null || groupLocations == null) {
    return <div>Loading...</div>;
  }

  return (
    <div className="bg-gray-800">
      <SkillTree baseSize={baseSize!} groupLocations={groupLocations!} />
    </div>
  );
}

export default App;
