import React from 'react';
import Calendar from 'react-calendar';
import '../../../App.css';

export default class CalendarView extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      date: new Date(),
    }

  }

  onChange = (date) => {
    this.setState({ date });
  }

  render() {
    return (
      <div className="App">
        <Calendar
          onChange={this.onChange}
          value={this.state.date}
          className="calendar"
        />
      </div>

    );

  }

}
