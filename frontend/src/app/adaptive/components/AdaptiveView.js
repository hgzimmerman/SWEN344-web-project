import React from 'react';
import CanvasJSReact from '../../../assets/canvasjs.react';
var CanvasJSChart = CanvasJSReact.CanvasJSChart;

export default class StocksView extends React.Component {
  dataPoints() {
    if (this.props.data != null) {
      let items =  this.props.data
        .map((item) => {
            return {
              x: new Date(item.time_recorded),
              y: item.did_serve ? 1 : 0
            }
          }
        );

      const interval = 60 * 60 * 1000; // 1 hour
      let groupTime = items[0].x.getTime();
      let groups = groupBy(items, d => {
        return d.x.getTime() - groupTime <= interval
          ? groupTime
          : groupTime = d.x.getTime();
      });

      let retval = [];
      for (let key in groups) {
        let time = key;
        let averageAvailability = groups[key].reduce((prev, curr) => {return prev + curr.y;}, 0)/groups[key].length;
        retval.push({
          x: time,
          y: averageAvailability
        });
      }
      // Parse the stringified dates back to dates, because apparently that's necessary.
      let reParsed = retval.map(ts => {
        return {
          x: new Date(Number(ts.x)),
          y: ts.y
        };
      });
      return reParsed;
    } else {
      return []
    }
  }





  render() {
    const dataPoints = this.dataPoints();
    // const dataPoints = [{"x":"2019-04-14T16:47:22.582Z","y":0.8},{"x":"2019-04-14T17:39:08.808Z","y":1}];//{"x":"2019-04-24T19:26:45.307Z","y":0.42857142857142855},{"x":"2019-04-25T00:55:36.121Z","y":1},{"x":"2019-04-25T02:25:21.641Z","y":1},{"x":"2019-04-25T04:03:07.441Z","y":1},{"x":"2019-04-25T05:59:22.998Z","y":1},{"x":"2019-04-25T11:45:34.752Z","y":1},{"x":"2019-04-25T12:09:13.997Z","y":0.42857142857142855},{"x":"2019-04-25T12:25:35.869Z","y":1},{"x":"2019-04-25T12:43:56.585Z","y":1},{"x":"2019-04-25T13:00:45.619Z","y":0.3333333333333333},{"x":"2019-04-25T13:17:48.580Z","y":0.6},{"x":"2019-04-25T13:35:03.632Z","y":1},{"x":"2019-04-25T16:11:59.152Z","y":1},{"x":"2019-04-25T16:52:46.551Z","y":0},{"x":"2019-04-26T14:15:36.373Z","y":1},{"x":"2019-04-26T15:36:04.625Z","y":1},{"x":"2019-04-26T21:38:33.427Z","y":0.6666666666666666},{"x":"2019-04-26T21:59:39.588Z","y":0},{"x":"2019-04-26T22:24:34.920Z","y":1},{"x":"2019-04-26T22:41:13.323Z","y":0},{"x":"2019-04-26T22:57:40.862Z","y":1},{"x":"2019-04-26T23:14:35.692Z","y":0.5},{"x":"2019-04-26T23:31:52.880Z","y":1},{"x":"2019-04-27T00:58:51.820Z","y":1},{"x":"2019-04-27T02:14:07.233Z","y":1}];
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
      <div id='Adaptive'>
        {
          (!this.props.isLoading)
            ? (this.props.error)
              ? <div> {this.props.error.message} </div>
              : (this.props.data !== undefined && this.props.data !== null)
                ? <CanvasJSChart options = {options} />
                : <></>
            : <div>Loading...</div>
        }
      </div>
    );
  }

}
const groupBy = (collection, iteratee = (x) => x) => {
  const it = typeof iteratee === 'function' ?
    iteratee : ({ [iteratee]: prop }) => prop;

  const array = Array.isArray(collection) ? collection : Object.values(collection);

  return array.reduce((r, e) => {
    const k = it(e);

    r[k] = r[k] || [];

    r[k].push(e);

    return r;
  }, {});
};