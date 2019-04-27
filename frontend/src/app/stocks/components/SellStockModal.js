import React from 'react';
import Button from '@material-ui/core/Button';
import TextField from '@material-ui/core/TextField';
import Modal from '@material-ui/core/Modal';


export default class SellStockModal extends React.Component {
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
    this.sellStock = this.sellStock.bind(this);
  }

  handleChangeQuantity(e) {
    this.setState({ quantity: e.target.value });

  }
  sellStock(symbol, quantity) {
    // Negate the quantity given to make this transaction a sale.
    let negatedQuantity = 0 - Number(quantity);
    // The negated value must be below 0.
    if (negatedQuantity <= 0) {
      alert("Quantity to sell must be positive");
    } else {
      this.transactStock(symbol, negatedQuantity)
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
              placeholder="Sell shares…"
              margin="normal"
              variant="outlined"
              onChange={this.handleChangeQuantity}
              style={{width: '50%'}}
            />
            <Button
              onClick={() => {
                this.closeModal();
                this.sellStock(
                  this.props.stock, this.state.quantity
                )
              }}
              variant="contained"
              style={styles.button}
            >
              {`Sell ${this.state.quantity} shares`}
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
    width: '300px',
    height: '320px',
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
