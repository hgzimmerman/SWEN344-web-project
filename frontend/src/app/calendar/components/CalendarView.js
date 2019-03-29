import React from 'react';
// import Calendar from 'react-calendar';
import BigCalendar from 'react-big-calendar'
import moment from 'moment'
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
    const localizer = BigCalendar.momentLocalizer(moment)
    return (
        <div>
          <BigCalendar
            localizer={localizer}
            events={['test']}
            startAccessor="start"
            endAccessor="end"
          />
        </div>
    );

  }

}
