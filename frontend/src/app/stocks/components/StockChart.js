import React from "react";
import { Chart } from "react-charts";

export default class StockChart extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      open: this.props.data
    }
  }

  feedAxis(){
    let arr = [];
    this.state.open.map((item) => (
      arr.push({x: item.date, y: item.high})
    ));
    return arr;
  }

  render(){
    const axis = this.feedAxis();
    const data = [
  {
    label: "Series 1",
    data: this.feedAxis()
  }
];
    return (
      // A react-chart hyper-responsively and continuusly fills the available
      // space of its parent element automatically
      <div
        style={{
          width: "400px",
          height: "300px"
        }}
      >
        <Chart
          data={data}
          axes={[
            { primary: true, type: "time", position: "bottom" },
            { type: "linear", position: "left" }
          ]}
        />
        <button onClick={() => console.log(axis)}>test</button>
      </div>
    );
  }
}
