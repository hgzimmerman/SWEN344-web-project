import React from 'react';
import { Link } from 'react-router-dom';
import AppBar from '@material-ui/core/AppBar';
import Typography from '@material-ui/core/Typography';
import IconButton from '@material-ui/core/IconButton';
import AccountCircle from '@material-ui/icons/AccountCircle';
import Menu from '@material-ui/core/Menu';
import Toolbar from '@material-ui/core/Toolbar';
import { AuthButton } from '../../../config/auth.js';
import { loggedIn } from '../../auth/components/Login.js';
import {isAuthenticated} from "../../../config/auth";

export default class CustomNavbar extends React.Component {
  state = {
    anchorEl: null,
  };

  handleMenu = event => {
    this.setState({ anchorEl: event.currentTarget });
  };

  handleClose = () => {
    this.setState({ anchorEl: null });
  };

  render(){
    const { anchorEl } = this.state;
    const open = Boolean(anchorEl);

    if (isAuthenticated()) {
      return(
        <nav>

          <AppBar position="static" style={styles.navbar} id='navbarRoot'>
            <Toolbar id='toolbarRoot'>
              <Link to="/" style={styles.tab} id='homeLinkContainer'>
                <Typography variant="h6" color="inherit" id='homeLink'>
                  Home
                </Typography>
              </Link>
              <Link to="/stocks" style={styles.tab} id='stockLinkContainer'>
                <Typography variant="h6" color="inherit" id='stockLink'>
                  Stocks
                </Typography>
              </Link>
              <Link to="/calendar" style={styles.tab} id='calendarLinkContainer'>
                <Typography variant="h6" color="inherit" id='calendarLink'>
                  Calendar
                </Typography>
              </Link>
              <Link to="/weather" style={styles.tab} id='weatherLinkContainer'>
                <Typography variant="h6" color="inherit" id='weatherLink'>
                  Weather
                </Typography>
              </Link>

              <Link to="/adaptive" style={styles.tab} id='adaptiveLinkContainer'>
                <Typography variant="h6" color="inherit" id='adaptiveLink'>
                  Adaptive
                </Typography>
              </Link>
              {isAuthenticated() && (

                <div style={{float: 'right'}} id='iconButtonContainer'>
                  <IconButton
                    id='iconButton'
                    aria-owns={open ? 'menu-appbar' : undefined}
                    aria-haspopup="true"
                    onClick={this.handleMenu}
                    color="inherit"
                  >
                    <AccountCircle id='accountCircle' />
                  </IconButton>
                  <Menu
                    id="menu-appbar"
                    anchorEl={anchorEl}
                    anchorOrigin={{
                      vertical: 'top',
                      horizontal: 'right',
                    }}
                    transformOrigin={{
                      vertical: 'top',
                      horizontal: 'right',
                    }}
                    open={open}
                    onClose={this.handleClose}
                    style={{marginTop: 40}}
                  >
                    <AuthButton id='authButton'/>
                  </Menu>
                </div>
              )}

            </Toolbar>
          </AppBar>
        </nav>
      );
    } else {
      return (
        <nav id='navRoot'>
          <AppBar position="static" style={styles.navbar} id='appBar'/>
        </nav>
      );
    }

  }

}

const styles = {
  navbar: {
    backgroundColor: '#00A6DD'
  },
  tab: {
    marginLeft: 20,
    marginRight: 30,
    textDecoration: 'none',
    color: 'white'
  }
}
