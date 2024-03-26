import React, { useEffect, useState } from 'react';
import Node, { TsNode } from './Node';
import { invokeAndSetWithArgs } from '../util/rustUtility';

interface GroupProps {
  groupLocations: TsGroupLocation;
}

export interface TsGroupLocation {
  group_id: number;
  x: number;
  y: number;
}

const Group: React.FC<GroupProps> = ({ groupLocations }) => {
  const [nodes, setNodes] = useState<TsNode[] | null>(null);

  useEffect(() => {
    invokeAndSetWithArgs(setNodes, 'get_nodes_for_group', {
      groupId: groupLocations.group_id,
    });
  }, []);

  if (nodes == null) {
    return <div>Loading...</div>;
  }

  return (
    <div
      key={`Group ${groupLocations.group_id}`}
      style={{ bottom: `${groupLocations.y}px`, left: `${groupLocations.x}px` }}
      className="absolute h-20  w-20 rounded-xl bg-white"
    >
      <div className="relative">
        {`Group ${groupLocations.group_id} ${groupLocations.x} ${groupLocations.y}`}
        {nodes.map((node) => (
          <Node
            key={node.node_id}
            nodeInfo={node}
            text={`Group ${groupLocations.group_id}`}
          />
        ))}
      </div>
    </div>
  );
};

export default Group;
