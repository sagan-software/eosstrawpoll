declare module 'url-template' {
    export default interface TemplateParser<P> {
        parse(template: string): Template<P>;
    }

    export interface Template<P> {
        expand(parameters: P): string;
    }
}
