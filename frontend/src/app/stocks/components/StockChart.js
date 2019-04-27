import React from "react";
import CanvasJSReact from '../../../assets/canvasjs.react';
var CanvasJSChart = CanvasJSReact.CanvasJSChart;

export default class StockChart extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      data: this.props.data
    }
  }

  feedData(){
    let arr = [];
    this.state.data.map((item) => {
      arr.push({x: new Date(parseInt(item.date.substr(0,4)), parseInt(item.date.substr(5,2)-1), parseInt(item.date.substr(8,2))), y: item.close})
    });
    return arr;
  }


  render() {
    const dataPoints = this.feedData();
		const options = {
			// width: 400,
			animationEnabled: true,
			title: {
				text: `${this.props.stock} Interactive Chart (Month)`
			},
			axisY: {
				title: "Closed Price",
				includeZero: false,
				prefix: "$"
			},
			data: [{
				type: "splineArea",
				xValueFormatString: "MM DD",
				yValueFormatString: "#,$##0.##",
				showInLegend: false,
				legendText: "Date",
				dataPoints: dataPoints
			}]
		};

		return (
		<div id='stockChartDiv'>
			<CanvasJSChart
        id='stockChartCanvas'
        options = {options} />
		</div>
		);
	}
}
