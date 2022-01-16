import React, {useState} from 'react'
import PropTypes from 'prop-types'
import axios from 'axios';

function AddTodo(props) {
    const [description, setDescription] = useState("");

    const handleSubmit = (evt) => {
        evt.preventDefault();
        axios.post('http://localhost:8000/tasks', {
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

}

export default AddTodo

