import React from 'react';

interface NodeProps {
  nodeInfo: TsNode;
}

export interface TsNode {
  node_id: number;
  node_name: string;
  x: number;
  y: number;
}

const Node: React.FC<NodeProps> = ({ nodeInfo }) => {
  return (
    <div
      key={`Node ${nodeInfo.node_id}`}
      style={{ bottom: `${nodeInfo.x}px`, left: `${nodeInfo.y}px` }}
      className="absolute h-10  w-10 rounded-xl bg-orange-400"
    >
      {`${nodeInfo.node_name}`}
    </div>
  );
};

export default Node;
