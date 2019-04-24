import React from 'react';
import CanvasJSReact from '../../../assets/canvasjs.react';
var CanvasJSChart = CanvasJSReact.CanvasJSChart;

export default class StocksView extends React.Component {
    constructor(props){
    super(props);
    this.state = {
      data: this.props.data,
      error: this.props.error,
      isLoading: this.props.isLoading,
    };
  }

  componentWillReceiveProps(newProps){
    this.setState({
      data: newProps.data,
      error: newProps.error,
      isLoading: newProps.isLoading
    })
  }

  dataPoints() {
    if (this.state.data != null) {
      const now = Date.now();
      return this.state.data.map((item) => {
          return {
            x: new Date(item.time_recorded),
            y: item.did_serve ? 1 : 0
          }
        }
      )
    } else {
      return null
    }
  }

  render() {
    console.log(JSON.stringify(this.state));
    const dataPoints = this.dataPoints();
		const options = {
			animationEnabled: true,
			title: {
				text: `Ad Liveness`
			},
			axisY: {
				title: "Is Up",
				includeZero: false,
        viewportMaximum: 1,
        viewportMinimum: 0
			},
			data: [{
				type: "splineArea",
        xValueFormatString: "DDD HH:MM",
				showInLegend: false,
				legendText: "Date",
				dataPoints: dataPoints
			}]
		};

    return (
      <div>
        {
          (!this.state.isLoading)
            ? (this.state.error)
              ? <div> {this.state.error.message} </div>
              : (this.state.data !== undefined && this.state.data !== null)
                ? <CanvasJSChart options = {options} />
                : <div></div>
            : <div>Loading...</div>
        }
      </div>
    );
  }

}
