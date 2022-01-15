import React from 'react'
import PropTypes from 'prop-types'

function TodoItem(props) {
    let task = props.task;

    const completedChange = (e) => {
        console.log(`completedChange: ${e.target.value}`);
    }

    return (
        <tr className="todo-item">
            <td>{task.description}</td>
            <td>{task.created_at}</td>
            <td><input type="checkbox"
                onChange={(e) => completedChange(e)}
                checked={task.completed} /></td>
            <td>❌</td>
        </tr>
    )
}

TodoItem.propTypes = {
    task: PropTypes.object.isRequired,
};

export default TodoItem;

