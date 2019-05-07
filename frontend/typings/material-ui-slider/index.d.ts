declare module 'material-ui-slider' {
    export interface SliderProps {
        color?: string;
        defaultValue?: number | number[];
        range?: boolean;
        min?: number;
        max?: number;
        value?: number | number[];
        scaleLength?: number;
        direction?: 'horizontal' | 'vertical';
        disabled?: boolean;
        onChange?: (e: number | number[]) => any;
        onChangeComplete?: (e: number | number[]) => any;
    }
    export class Slider extends React.Component<SliderProps> {}
}
