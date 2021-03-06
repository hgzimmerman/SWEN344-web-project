import React from 'react';
import { Link } from 'react-router-dom';
import AppBar from '@material-ui/core/AppBar';
import Typography from '@material-ui/core/Typography';
import IconButton from '@material-ui/core/IconButton';
import AccountCircle from '@material-ui/icons/AccountCircle';
import Menu from '@material-ui/core/Menu';
import Toolbar from '@material-ui/core/Toolbar';
import { AuthButton, fakeAuth } from '../../../config/auth.js';

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
    return(
      <div>
        <AppBar position="static" style={styles.navbar}>
          {
            (fakeAuth.isAuthenticated)
            ? <Toolbar>
                <Link to="/" style={styles.tab}>
                  <Typography variant="h6" color="inherit">
                    Home
                  </Typography>
                </Link>
                <Link to="/stocks" style={styles.tab}>
                  <Typography variant="h6" color="inherit">
                    Stocks
                  </Typography>
                </Link>
                <Link to="/calendar" style={styles.tab}>
                  <Typography variant="h6" color="inherit">
                    Calendar
                  </Typography>
                </Link>
                <Link to="/weather" style={styles.tab}>
                  <Typography variant="h6" color="inherit">
                    Weather
                  </Typography>
                </Link>

                <div style={{float: 'right'}}>
                  <IconButton
                    aria-owns={open ? 'menu-appbar' : undefined}
                    aria-haspopup="true"
                    onClick={this.handleMenu}
                    color="inherit"
                  >
                    <AccountCircle />
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
                    <AuthButton />
                  </Menu>
                </div>


              </Toolbar>
            : <Toolbar></Toolbar>
          }
        </AppBar>
      </div>
    );

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
