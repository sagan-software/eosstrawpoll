import React from 'react';
import TextField from '@material-ui/core/TextField';
import InputAdornment from '@material-ui/core/InputAdornment';
import IconButton from '@material-ui/core/IconButton';
import Typography from '@material-ui/core/Typography';
import DeleteIcon from '@material-ui/icons/Delete';
import Button from '@material-ui/core/Button';
import { useService } from '@xstate/react';
import ExpansionPanel from '@material-ui/core/ExpansionPanel';
import ExpansionPanelSummary from '@material-ui/core/ExpansionPanelSummary';
import ExpansionPanelDetails from '@material-ui/core/ExpansionPanelDetails';
import FormGroup from '@material-ui/core/FormGroup';
import FormControlLabel from '@material-ui/core/FormControlLabel';
import Switch from '@material-ui/core/Switch';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import ChipInput from 'material-ui-chip-input';
import CircularProgress from '@material-ui/core/CircularProgress';
import Box from '@material-ui/core/Box';
import DateFnsUtils from '@date-io/date-fns';
import { Slider } from 'material-ui-slider';
import { DateTimePicker, MuiPickersUtilsProvider } from '@material-ui/pickers';

import * as pollForm from '../machines/pollForm';

export default function PollForm() {
    const [current, send] = useService(pollForm.service);
    const lastIndex = current.context.options.length - 1;
    const isSubmitting = current.matches('submitting');
    return (
        <MuiPickersUtilsProvider utils={DateFnsUtils}>
            <form
                onSubmit={(e) => {
                    e.preventDefault();
                    send({
                        type: 'SUBMIT',
                    });
                }}
            >
                <ExpansionPanel expanded>
                    <ExpansionPanelDetails>
                        <div>
                            <TextField
                                placeholder='What would you like to ask?'
                                margin='normal'
                                variant='outlined'
                                autoFocus
                                fullWidth
                                value={current.context.name}
                                onChange={(e) =>
                                    send({
                                        type: 'SET_NAME',
                                        value: e.target.value,
                                    })
                                }
                            />

                            {current.context.options.map((option, index) => (
                                <TextField
                                    variant='outlined'
                                    fullWidth
                                    margin='dense'
                                    placeholder={`Option ${index + 1}`}
                                    value={option}
                                    onChange={(e) => {
                                        send({
                                            type: 'SET_OPTION',
                                            index,
                                            value: e.target.value,
                                        });
                                    }}
                                    onFocus={(_e) => {
                                        if (index === lastIndex) {
                                            send({ type: 'ADD_OPTION' });
                                        }
                                    }}
                                    InputProps={{
                                        endAdornment: (
                                            <InputAdornment position='end'>
                                                <IconButton
                                                    aria-label='Delete'
                                                    edge='end'
                                                    onClick={(_e) =>
                                                        send({
                                                            type: 'DEL_OPTION',
                                                            index,
                                                        })
                                                    }
                                                    disabled={lastIndex <= 1}
                                                >
                                                    <DeleteIcon fontSize='small' />
                                                </IconButton>
                                            </InputAdornment>
                                        ),
                                    }}
                                />
                            ))}
                        </div>
                    </ExpansionPanelDetails>
                </ExpansionPanel>

                <ExpansionPanel>
                    <ExpansionPanelSummary
                        expandIcon={<ExpandMoreIcon />}
                        aria-controls='panel2a-content'
                        id='panel2a-header'
                    >
                        <Typography>Answer Requirements</Typography>
                    </ExpansionPanelSummary>
                    <ExpansionPanelDetails>
                        <Box width={1}>
                            <Slider
                                color='#bf4040'
                                min={1}
                                max={2}
                                defaultValue={[1, 1]}
                                range
                                scaleLength={1}
                            />
                        </Box>
                    </ExpansionPanelDetails>
                </ExpansionPanel>

                <ExpansionPanel>
                    <ExpansionPanelSummary
                        expandIcon={<ExpandMoreIcon />}
                        aria-controls='panel2a-content'
                        id='panel2a-header'
                    >
                        <Typography>Voter Requirements</Typography>
                    </ExpansionPanelSummary>
                    <ExpansionPanelDetails>
                        <Box width={1}>
                            <FormControlLabel
                                control={
                                    <Switch value='checkedB' color='primary' />
                                }
                                label='Allow list'
                            />
                            <ChipInput
                                classes={{}}
                                defaultValue={['foo', 'bar']}
                                onChange={(chips) => console.log(chips)}
                                fullWidth
                                helperText='Only these accounts will be allowed to vote'
                            />
                        </Box>
                    </ExpansionPanelDetails>
                </ExpansionPanel>

                <ExpansionPanel>
                    <ExpansionPanelSummary
                        expandIcon={<ExpandMoreIcon />}
                        aria-controls='panel2a-content'
                        id='panel2a-header'
                    >
                        <Typography>Dates</Typography>
                    </ExpansionPanelSummary>
                    <ExpansionPanelDetails>
                        <div>
                            <DateTimePicker
                                autoOk
                                disablePast
                                label='Open Time'
                                onChange={(value) => {
                                    send({
                                        type: 'SET_OPEN_TIME',
                                        value: value as Date,
                                    });
                                }}
                                value={current.context.openTime}
                            />
                            <DateTimePicker
                                autoOk
                                disablePast
                                label='Close Time'
                                onChange={(value) => {
                                    send({
                                        type: 'SET_CLOSE_TIME',
                                        value: value as Date,
                                    });
                                }}
                                value={current.context.closeTime}
                            />
                        </div>
                    </ExpansionPanelDetails>
                </ExpansionPanel>

                <Box
                    display='flex'
                    alignItems='center'
                    justifyContent='center'
                    m={2}
                >
                    <Button
                        variant='contained'
                        color='primary'
                        size='large'
                        type='submit'
                    >
                        Create Poll
                    </Button>
                </Box>
            </form>
        </MuiPickersUtilsProvider>
    );
}
