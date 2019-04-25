import React from 'react';
import FullCalendar from 'fullcalendar-reactwrapper';
import BigCalendar from 'react-big-calendar';
import '../../../../node_modules/fullcalendar-reactwrapper/dist/css/fullcalendar.min.css';
import moment from 'moment'
import '../../../App.css';

export default class CalendarView extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      date: new Date(),
      events: null
    }

  }

  onChange = (date) => {
    this.setState({ date });
  }

  render() {
    const localizer = BigCalendar.momentLocalizer(moment)
    return (
          <FullCalendar
            id="main-calendar"
            header={{
              left: 'prev, next today myCustomButton',
              center: 'title',
              right: 'month,basicWeek,basicDay'
            }}
            navLinks={true}
            editable={true}
            eventLimit={true}
            events={this.state.events}
          />
    );

  }

}
