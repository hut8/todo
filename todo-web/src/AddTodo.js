import React, {useState} from 'react'
// import PropTypes from 'prop-types'
import axios from 'axios';
import Settings from './Settings';

function AddTodo(props) {
    const [description, setDescription] = useState("");

    const handleSubmit = (evt) => {
        const url = new URL(Settings.apiBase);
        evt.preventDefault();
        axios.post(url, {
            description
        }, {crossdomain:true});
    };

    return (
        <div className='row'>
            <form autoComplete='off' onSubmit={handleSubmit}>
                <input
                    value={description} 
                    onChange={(e) => setDescription(e.target.value)}
                    name="description" 
                    placeholder='New Task...'
                    />
                <input type="submit" />
            </form>
        </div>
    )
}

AddTodo.propTypes = {

};

export default AddTodo;

