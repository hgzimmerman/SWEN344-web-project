import React from 'react';
import FullCalendar from 'fullcalendar-reactwrapper';
import BigCalendar from 'react-big-calendar';
import Button from '@material-ui/core/Button';
import '../../../../node_modules/fullcalendar-reactwrapper/dist/css/fullcalendar.min.css';
import '../../../../node_modules/react-datetime/css/react-datetime.css';
import moment from 'moment';
import '../../../App.css';
import Datetime from 'react-datetime';
import { authenticatedFetchDe } from "../../../config/auth";

export default class CalendarView extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      date: new Date().toJSON(),
      events: null
    }
    this.createEvent = this.createEvent.bind(this);
    this.getEvents = this.getEvents.bind(this);

  }

  componentDidMount(){
    this.getEvents();

  }

  onChange = (date) => {
    this.setState({ date });
    console.log(date.toJSON());

  }

  getEvents() {
    let now = new Date();
    let midnight = new Date();
    midnight.setHours(24,0,0,0);
    const url = `api/calendar/events`;
    return authenticatedFetchDe(url)
      .then(res => {
        this.setState({
          events: res
        })
      });

  }

  createEvent(){
    const url = '/api/calendar/event/';
    const body = JSON.stringify({
      "Event": (this.state.date).toJSON()
    });
    authenticatedFetchDe(url, {method: "POST", body})
      .then(() => {
        this.getEvents() // Update the whole stock table after performing a sale or purchase.
      })
  }

  render() {
    const localizer = BigCalendar.momentLocalizer(moment)
    return (
      <div style={{padding: 20}}>
        <div style={{padding: 20}}>
          <h3>Create an event</h3>
          <Datetime onChange={this.onChange}/>
          <Button
            onClick={() => this.createEvent()}
            variant="contained"
            style={styles.button}
          >
            Create
          </Button>
        </div>
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
      </div>
    );

  }

}

const styles = {
  button: {
    backgroundColor: '#00A6DD',
    color: 'white',
    height: 40,
    width: 130,
    marginTop: 20
  },

}
