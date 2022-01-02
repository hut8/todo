import React, {useState} from 'react'
import PropTypes from 'prop-types'

function AddTodo(props) {
    const [description, setDescription] = useState("");
    const handleSubmit = (evt) => {
        evt.preventDefault();
        alert("Submitting");
    }
    return (
        <div>
            <form onSubmit={handleSubmit}>
                <input
                    value={description} 
                    onChange={(e) => setDescription(e.target.value)}
                    name="description" />
                <input type="submit" />
            </form>
        </div>
    )
}

AddTodo.propTypes = {

}

export default AddTodo

