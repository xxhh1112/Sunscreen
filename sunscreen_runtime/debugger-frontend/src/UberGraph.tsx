import { GraphView } from "react-digraph";
import React from "react";

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faDownLeftAndUpRightToCenter } from '@fortawesome/free-solid-svg-icons'

const GraphConfig =  {
  NodeTypes: {
    empty: { // required to show empty nodes
      typeText: "None",
      shapeId: "#empty", // relates to the type property of a node
      shape: (
        <symbol viewBox="0 0 100 100" id="empty" key="0">
          <circle cx="50" cy="50" r="45"></circle>
        </symbol>
      )
    },
    inputCiphertext: { // required to show empty nodes
      typeText: "Ciphertext Input",
      shapeId: "#inputCiphertext", // relates to the type property of a node
      shape: (
        <symbol viewBox="0 0 90 50" id="inputCiphertext" key="0">
          <rect cx='50' cy='50' width='90' height='50'/>
        </symbol>
      )
    },
    inputPlaintext: {
      typeText: "Plain Input",
      shapeId: '#inputPlaintext',
      shape: (
        <symbol viewBox="0 0 100 100" id="inputCiphertext" key="0">
          <ellipse cx='50' cy='50' width='90' height='50'/>
        </symbol>
      )
    },
    outputCiphertext: { // required to show empty nodes
      typeText: "Output",
      shapeId: "#outputCiphertext", // relates to the type property of a node
      shape: (
        <symbol viewBox="0 0 100 100" id="outputCiphertext" key="0">
          <circle cx="50" cy="50" r="45"></circle>
        </symbol>
      )
    },
    add: {
      typeText: "+",
      shapeId: "#add",
      shape: (
        <symbol viewBox="0 0 50 50" id="add" key="0" fontSize="18pt" fill='aquamarine'>
          <svg viewBox="0 0 50 50">
            <circle cx="25" cy="25" r="12" fill="aquamarine" strokeWidth="1.5"></circle>
            <line x1="17.5" y1="25" x2="32.5" y2="25" strokeWidth="3.5"></line>
            <line x1="25" y1="17.5" x2="25" y2="32.5" strokeWidth="3.5"></line>
          </svg>

        </symbol>
      )
    },
    multiply: {
      typeText: "*",
      shapeId: "#multiply",
      shape: <symbol viewBox="0 0 50 50" id="multiply" key="0" fontSize="18pt">
      <svg viewBox="0 0 50 50">
        <circle cx="25" cy="25" r="12" fill="khaki" strokeWidth="1.5"></circle>
        <line x1="20" y1="20" x2="30" y2="30" strokeWidth="3.5"></line>
        <line x1="30" y1="20" x2="20" y2="30" strokeWidth="3.5"></line>
      </svg>

    </symbol>
    },
    sub: {
      typeText: "",
      shapeId: "#sub",
      shape: (
        <symbol viewBox="0 0 50 50" id="sub" key="0" fontSize="18pt" fill='plum'>
          <svg viewBox="0 0 50 50">
            <circle cx="25" cy="25" r="12" fill="plum" strokeWidth="1.5"></circle>
            <line x1="17.5" y1="25" x2="32.5" y2="25" strokeWidth="3.5"></line>
          </svg>

        </symbol>
      )
    },
    constraint: {
      typeText: "",
      shapeId: "#constraint",
      shape: (
        <symbol viewBox="0 0 50 50" id="constraint" key="0" fontSize="18pt" fill='plum'>
          <svg viewBox="0 0 50 50">
            <circle cx="25" cy="25" r="12" fill="lime" strokeWidth="1.5"></circle>
            <line x1="17.5" y1="29" x2="32.5" y2="29" strokeWidth="3.5"></line>
            <line x1="17.5" y1="21" x2="32.5" y2="21" strokeWidth="3.5"></line>
          </svg>
        </symbol>
      )
    },
    relinearize: {
      typeText: "Relin",
      shapeId: "#relinearize",
      shape: (
        <symbol viewBox="0 0 50 50" id="relinearize" key="0">
          <svg viewBox="0 0 50 50">
            <circle cx="25" cy="25" r="12">
            </circle>
            {/* <defs>
              <marker id='head' orient="auto"
                markerWidth='1.5' markerHeight='4'
                refX='0.1' refY='2'>
                <path d='M0,0 V4 L2,2 Z' fill="black"/>
              </marker>
            </defs>

            <path
              id='arrow-line'
              marker-end='url(#head)'
              stroke-width='1.5'
              fill='none' stroke='black'  
              d='M20,20 23,23'
              />
            <path
              id='arrow-line'
              marker-end='url(#head)'
              stroke-width='1.5'
              fill='none' stroke='black'  
              d='M30,30 28,28'
              /> */}
          </svg>
          
        </symbol>
      )
    },
    problematic: {
      typeText: "Problematic",
      shapeId: "#problem", // relates to the type property of a node
      shape: (
        <symbol viewBox="0 0 100 100" id="problem" key="0">
          <circle cx="50" cy="50" r="45" fill='pink'></circle>
        </symbol>
      )
    }
  },
  NodeSubtypes: {},
  EdgeTypes: {
    emptyEdge: {  // required to show empty edges
      shapeId: "#emptyEdge",
      shape: (
        <symbol viewBox="0 0 50 50" id="emptyEdge" key="0">
          <circle cx="25" cy="25" r="8" fill="currentColor"> </circle>
        </symbol>
      )
    }
  }
}

function UberGraph({graph, onSelect, selected}) {
  // const [selected, select] = useState(null);
  console.log('render')
  return (
  <GraphView
    nodeKey="id"
    nodes={graph.nodes}
    edges={graph.edges}
    allowMultiselect={false}
    layoutEngineType='VerticalTree'
    readOnly={true}
    nodeTypes={GraphConfig.NodeTypes}
    edgeTypes={GraphConfig.EdgeTypes}
    nodeSubtypes={GraphConfig.NodeSubtypes}
    onCreateNode={() => {}}
    selected={selected}
    onSwapEdge={() => {}}
    onCreateEdge={() => {}}
    onSelect={onSelect}
  />)
}
export {UberGraph};

