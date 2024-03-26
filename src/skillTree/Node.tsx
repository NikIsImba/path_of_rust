import React from 'react';

interface NodeProps {
  nodeInfo: TsNode;
  text: string;
}

export interface TsNode {
  node_id: number;
  node_name: string;
  x: number;
  y: number;
}

const Node: React.FC<NodeProps> = ({ nodeInfo, text }) => {
  return (
    <div
      key={`Node ${nodeInfo.node_id}`}
      style={{ bottom: `${nodeInfo.y}px`, left: `${nodeInfo.x}px` }}
      className="absolute h-10  w-10 rounded-xl bg-orange-400"
    >
      {`${text} `}
      {`${nodeInfo.node_name}`}
    </div>
  );
};

export default Node;
