import React from 'react';
import Button from '@material-ui/core/Button';
import TextField from '@material-ui/core/TextField';
import Modal from '@material-ui/core/Modal';


export default class BuyStockModal extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      visible: this.props.visible,
      stock: this.props.stock,
      quantity: ''
    };
    this.handleChangeQuantity = this.handleChangeQuantity.bind(this);
    this.closeModal = this.props.closeModal.bind(this);
    this.transactStock = this.props.transactStock.bind(this);
    this.buyStock = this.buyStock.bind(this);
  }

  handleChangeQuantity(e) {
    this.setState({ quantity: e.target.value });
  }

  buyStock(symbol, quantity) {
    if (quantity < 0) {
      alert("Must have a positive quantity");
    } else {
      this.transactStock(symbol, quantity)
    }
  }


  render(){
    return (
        <Modal
          aria-labelledby="simple-modal-title"
          aria-describedby="simple-modal-description"
          open={this.state.visible}
          onClose={this.closeModal}
        >
          <div style={styles.paper}>
            <h2 variant="h6" id="modal-title">
              {`${this.props.stock} stock`}
            </h2>
            <TextField
              id="outlined-with-placeholder"
              label="Quantity"
              placeholder="Buy shares…"
              margin="normal"
              variant="outlined"
              onChange={this.handleChangeQuantity}
              style={{width: '50%'}}
            />
            <Button
              onClick={() => {
                this.closeModal();
                this.buyStock(
                  this.props.stock, this.state.quantity
                )
              }}
              variant="contained"
              style={styles.button}
            >
              {`Buy ${this.state.quantity} shares`}
            </Button>
          </div>
        </Modal>
    );
  }
}

const styles = {
  paper: {
    position: 'absolute',
    left: '40%',
    top: '20%',
    textAlign: 'center',
    backgroundColor: 'white',
    width: '20%',
    height: '30%',
    outline: 'none',
    boxShadow: 3,
    borderRadius: 8,

  },
  button: {
    backgroundColor: '#00A6DD',
    color: 'white',
    height: 50,
    width: 200,
    marginTop: 30
  },
};
